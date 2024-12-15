<script lang="ts">
  import { listSnippets, type Snippet } from "../api"
  import { Search } from "lucide-svelte"

  const PAGE_SIZE = 10
  const MIN_SCROLL_REMAINING_PX = 1000

  type List = {
    search: string
    snippets: Snippet[]
    nextId: number | null
    activeIndex: number
    initialized: boolean
  }

  let inputRef: HTMLInputElement
  let listRef: HTMLDivElement
  let itemRefs: HTMLDivElement[] = []

  let list: List = $state({
    search: "",
    snippets: [],
    nextId: null,
    activeIndex: 0,
    initialized: false
  })
  let pageLock: AbortController | null = null

  const maybeLoadNextPage = async () => {
    const { search, nextId, initialized } = list

    if (!initialized) {
      if (!pageLock?.signal.aborted) return
      if (nextId === null) return
    }

    pageLock = new AbortController()
    const { signal } = pageLock

    const page = await listSnippets({
      search,
      startId: nextId,
      limit: PAGE_SIZE
    })

    if (signal.aborted) return

    list.nextId = page.nextId
    list.snippets.push(...page.snippets)
  }

  const getRemainingScrollPx = ({ scrollHeight, scrollTop, clientHeight }: HTMLElement) => {
    return scrollHeight - scrollTop - clientHeight
  }

  const setActiveIndex = (index: number) => {
    list.activeIndex = index
    itemRefs[index].scrollIntoView({ behavior: "smooth", block: "end", inline: "nearest" })
  }

  $effect(() => {
    inputRef.focus()

    maybeLoadNextPage()

    listRef.addEventListener("scroll", async () => {
      const remainingPx = getRemainingScrollPx(listRef)
      if (MIN_SCROLL_REMAINING_PX <= remainingPx) return

      maybeLoadNextPage()
    })
  })
</script>

<div class="flex flex-col h-screen">
  <div class="flex items-center h-8 px-2 gap-2" style="background-color: rgba(41, 41, 42, .6)">
    <Search class="text-white opacity-50 " size={15} />
    <input
      bind:this={inputRef}
      class="font-mono bg-transparent text-white placeholder-gray-400 focus:outline-none flex-grow text-sm"
      type="text"
      placeholder="search code"
      spellCheck={false}
      autoComplete="off"
      autoCorrect="off"
      autocapitalize="off"
      autocomplete="off"
      value={list.search}
      oninput={async (event) => {
        const { value } = event.currentTarget

        list = {
          search: value,
          nextId: null,
          snippets: [],
          activeIndex: 0,
          initialized: false
        }

        maybeLoadNextPage()
      }}
    />
  </div>
  <div
    class="snippet-list flex overflow-y-auto flex-1 flex-col whitespace-nowrap"
    style="background-color: rgba(41, 41, 42, .4)"
    bind:this={listRef}
  >
    {#if list}
      {#each list.snippets as { preview_html }, i}
        <div
          bind:this={itemRefs[i]}
          class={`text-xs p-2 ${i === list.activeIndex && "bg-white/10"} hover:bg-white/5`}
          role="none"
          onclick={() => {}}
        >
          <pre>{@html preview_html}</pre>
        </div>
        <div class="border-t border-white/15"></div>
      {/each}
    {/if}
  </div>
</div>

<svelte:window
  on:keydown={({ key }) => {
    if (!list) return
    const { activeIndex, snippets } = list

    switch (key) {
      case "ArrowUp": {
        const i = Math.max(activeIndex - 1, 0)
        setActiveIndex(i)
        break
      }
      case "ArrowDown": {
        const i = Math.min(activeIndex + 1, snippets.length - 1)
        setActiveIndex(i)
        break
      }
      case "ArrowRight":
        console.log("right")
    }
  }}
/>

<style>
  .snippet-list {
    overflow-x: hidden;
    scrollbar-width: thin;
    scrollbar-color: rgba(0, 0, 0, 0.5) rgba(0, 0, 0, 0.1);
  }

  .snippet-list::-webkit-scrollbar {
    width: 10px;
    height: 15px;
  }

  .snippet-list::-webkit-scrollbar-track {
    background-color: rgba(0, 0, 0, 0.1);
  }

  .snippet-list::-webkit-scrollbar-thumb {
    background-color: rgba(0, 0, 0, 0.2);
    border-radius: 4px;
  }
</style>
