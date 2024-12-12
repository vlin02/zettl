<script lang="ts">
    import { listSnippets } from "./commands"

  let count: number = $state(0)
  const increment = () => {
    listSnippets({search: "wef"}).then(console.log)
    count += 1
  }
</script>

<button onclick={increment}>
  count is {count}
</button>
