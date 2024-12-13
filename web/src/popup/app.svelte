<script lang="ts">
  import { listSnippets, type Snippet } from "../api"
  import { Search } from "lucide-svelte"

  let search: string = $state("")
  let snippets: Snippet[] = $state([])

  let timeout: number | null = $state(null)

  const increment = () => {
    listSnippets({ search: "a" }).then((x) => console.log(JSON.stringify(x, undefined, 2)))
  }
</script>

<div class="flex flex-col h-screen">
  <div class="flex items-center h-8 px-2 gap-2" style="background-color: rgba(41, 41, 42, .6)">
    <Search class="text-white opacity-50 " size={15} />
    <input
      bind:value={search}
      oninput={() => {
        if (timeout) {
          clearTimeout(timeout)
        }

        timeout = window.setTimeout(async () => {
          snippets = await listSnippets({ search })
        }, 50)
      }}
      type="text"
      placeholder="search code"
      class="font-mono bg-transparent text-white placeholder-gray-400 focus:outline-none flex-grow text-sm"
    />
  </div>
  <div class="snippet-list flex overflow-y-auto flex-1 flex-col whitespace-nowrap" style="background-color: rgba(41, 41, 42, .4)">
    {#each snippets as { preview_html }}
      <div class="text-xs p-2">
        <pre>{@html preview_html}</pre>
      </div>
      <div class="border-t border-white/[.1]"></div>
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
