<script lang="ts">
  import { listen } from "@tauri-apps/api/event"
  import { closeWindow, copySnippet, listSnippets, type Snippet } from "../api"
  import { Search } from "lucide-svelte"
  import { untrack } from "svelte"

  const PAGE_SIZE = 50
  const MIN_SCROLL_REMAINING_PX = 1000

  type List = {
    search: string
    snippets: Snippet[]
    nextId: number | null
    activeIndex: number
  }

  const initialList: List = {
    search: "",
    snippets: [],
    nextId: null,
    activeIndex: 0
  }

  let inputRef: HTMLInputElement
  let listRef: HTMLDivElement
  let itemRefs: HTMLDivElement[] = $state([])

  let activeKeys: string[] = $state([])

  let list: List = $state(initialList)
  let pageLock: AbortController | null = $state.raw(null)

  const maybeLoadNextPage = async (search?: string) => {
    if (search !== undefined) {
      pageLock?.abort()
      list.search = search
    } else {
      if (pageLock && !pageLock.signal.aborted) return
      if (list.nextId === null) return
    }

    pageLock = new AbortController()
    const { signal } = pageLock

    const page = await listSnippets({
      search: search ?? list.search,
      startId: search !== undefined ? null : list.nextId,
      limit: PAGE_SIZE
    })

    if (signal.aborted) return

    if (search !== undefined) {
      list.snippets = []
      list.activeIndex = 0
    }
    list.nextId = page.nextId
    list.snippets.push(...page.snippets)

    pageLock = null
  }

  const getRemainingScrollPx = ({ scrollHeight, scrollTop, clientHeight }: HTMLElement) => {
    return scrollHeight - scrollTop - clientHeight
  }

  const selectIndex = $derived((index: number) => {
    list.activeIndex = index
    itemRefs[index].scrollIntoView({ behavior: "smooth", block: "center", inline: "nearest" })
  })

  $effect(() => {
    inputRef.focus()
  })

  $effect(() => {
    const onScroll = async () => {
      const remainingPx = getRemainingScrollPx(listRef)
      if (MIN_SCROLL_REMAINING_PX <= remainingPx) return
      maybeLoadNextPage()
    }

    listRef.addEventListener("scroll", onScroll)

    return () => {
      listRef.removeEventListener("scroll", onScroll)
    }
  })

  $effect(() => {
    untrack(() => maybeLoadNextPage(""))
  })

  $effect(() => {
    listen<string>("content-copied", () => {
      maybeLoadNextPage("")
    })
  })
</script>

<svelte:window
  onkeydown={async ({ key, repeat }) => {
    const { activeIndex, snippets } = list

    if (!repeat) {
      activeKeys.push(key)
    }

    switch (activeKeys.join("+")) {
      case "Meta+l":
        inputRef.focus()
        break
      case "Enter":
        const snippet = snippets[activeIndex]
        if (!snippet) break

        await copySnippet(snippet.id)
        await closeWindow()
        break
      case "ArrowUp": {
        const i = Math.max(activeIndex - 1, 0)
        selectIndex(i)
        break
      }
      case "ArrowDown": {
        const i = Math.min(activeIndex + 1, snippets.length - 1)
        selectIndex(i)
        break
      }
    }
  }}
  onkeyup={({ key }) => {
    activeKeys = activeKeys.filter((x) => x !== key)
  }}
/>

<div class="flex h-screen flex-col">
  <div class="bg-primary/60 flex h-8 items-center gap-2 px-2">
    <Search class="text-white/50 " size={15} />
    <input
      bind:this={inputRef}
      class="flex-grow bg-transparent font-mono text-sm text-white placeholder-white/50 focus:outline-none"
      type="text"
      placeholder="search code"
      spellCheck={false}
      autoComplete="off"
      autoCorrect="off"
      autocapitalize="off"
      autocomplete="off"
      value={list.search}
      oninput={({ currentTarget: { value } }) => {
        maybeLoadNextPage(value)
      }}
    />
  </div>
  <div
    class="snippet-list bg-primary/40 flex flex-1 flex-col overflow-y-auto whitespace-nowrap"
    bind:this={listRef}
  >
    {#each list.snippets as { previewHtml }, i}
      <div
        bind:this={itemRefs[i]}
        class={`cursor-pointer p-2 text-xs ${i === list.activeIndex && "bg-white/10"} hover:bg-white/5`}
        role="none"
        onclick={() => {
          selectIndex(i)
        }}
      >
        <pre>{@html previewHtml}</pre>
      </div>
      <div class="border-t border-white/15"></div>
    {/each}
  </div>
</div>

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
