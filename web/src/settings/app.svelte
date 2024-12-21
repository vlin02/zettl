<script lang="ts">
  import "../app.css"

  import { Check, Plus, X } from "lucide-svelte"
  import { deleteTheme, getSettings, importTheme, updateUser, type Settings } from "../api"

  let settings: Settings | null = $state(null)
  let tab = $state<"general" | "theme" | "shortcuts">("theme")

  let themeInputRef: HTMLInputElement

  let tabOptions = $derived(
    (
      [
        { name: "General", value: "general" },
        { name: "Theme", value: "theme" },
        { name: "Shortcuts", value: "shortcuts" }
      ] as const
    ).map(({ name, value }) => {
      return { name, value, active: value === tab } as const
    })
  )

  let refetch = async () => {
    settings = await getSettings()
  }

  $effect(() => {
    refetch()
  })
</script>

{#if settings}
  <div class="flex flex-col gap-4 p-6">
    <div class="flex justify-center">
      <div class="bg-neutral-bg flex rounded-sm p-0.5">
        {#each tabOptions as { name, value, active }}
          <div
            class={`flex w-24 justify-center rounded-sm py-0.5 ${active && "bg-background"} ${!active && "cursor-pointer"}`}
            role="none"
            onclick={() => {
              tab = value
            }}
          >
            <p class="text-foreground text-xs">{name}</p>
          </div>
        {/each}
      </div>
    </div>
    {#if tab === "general"}
      <div class="flex justify-between">
        <div class="flex flex-col">
          <p class="text-foreground text-xs">Popup size</p>
          <p class="text-foreground text-neutral text-xs">Size of popup window on start</p>
        </div>
        <div class="flex items-center gap-1.5">
          <input
            class="bg-neutral-bg text-foreground w-12 rounded-sm p-1 px-1.5 text-xs"
            value={settings.user.popup_width}
          />
          <p class="text-foreground text-xs"><X size={12} /></p>
          <input
            class="bg-neutral-bg text-foreground w-12 rounded-sm p-1 px-1.5 text-xs"
            value={settings.user.popup_width}
          />
        </div>
      </div>
      <div class="flex justify-between">
        <div class="flex flex-col">
          <p class="text-foreground text-xs">Transparency</p>
          <p class="text-foreground text-neutral text-xs">Enable transparent popup background</p>
        </div>
        <label class="inline-flex cursor-pointer items-center">
          <input type="checkbox" value="" class="peer sr-only" />
          <div
            class="bg-neutral-bg peer-checked:bg-primary peer relative h-5 w-9 rounded-full after:absolute after:start-[2px] after:top-[2px] after:h-4 after:w-4 after:rounded-full after:border after:border-white after:bg-white peer-checked:after:translate-x-full"
          ></div>
        </label>
      </div>
    {/if}
    {#if tab === "theme"}
      <div class="flex flex-col rounded-sm">
        {#each settings.themes as { id, name, active, can_delete, preview_colors }}
          <div class="bg-neutral-bg flex">
            <div
              class="hover:bg-neutral flex flex-1 cursor-pointer items-center justify-between p-2"
              role="none"
              onclick={async () => {
                const { user } = settings!
                await updateUser({
                  ...user,
                  theme_id: id
                })
              }}
            >
              <div class="flex items-center gap-4">
                <div class="w-3">
                  {#if active}<Check class="text-foreground" size={12} />{/if}
                </div>
                <div class="flex">
                  {#each preview_colors as { r, g, b, a }}
                    <div
                      class="h-4 w-3"
                      style={`background-color: rgba(${r},${g},${b},${a})`}
                    ></div>
                  {/each}
                </div>
              </div>
              <p class="text-foreground text-xs">{name}</p>
            </div>
            <div
              class={`${can_delete && "hover:bg-neutral"} flex w-8 cursor-pointer items-center justify-center`}
              role="none"
              onclick={async () => {
                if (!can_delete) return

                await deleteTheme(id)
                await refetch()
              }}
            >
              <X class={`${can_delete ? "text-foreground" : "text-neutral"}`} size={12} />
            </div>
          </div>
        {/each}
        <div
          class="bg-neutral-bg hover:bg-neutral flex cursor-pointer"
          role="none"
          onclick={() => {
            themeInputRef.click()
          }}
        >
          <div class="flex flex-1 items-center justify-between p-2">
            <div class="flex items-center gap-4">
              <div class="w-3"></div>
              <div class="flex">
                {#each ["#1c1917", "#44403c", "#57534e", "#78716c", "#a8a29e"] as c}
                  <div class="h-4 w-3" style={`background-color: ${c}`}></div>
                {/each}
              </div>
            </div>
            <div class="flex items-center gap-2">
              <div class="flex items-center gap-1">
                <Plus class="text-neutral" size={12} />
                <p class="text-neutral text-xs">Import</p>
              </div>
            </div>
          </div>
          <div class="w-8"></div>
        </div>
      </div>
      <input
        bind:this={themeInputRef}
        type="file"
        class="hidden"
        onchange={async (e) => {
          const file = e.currentTarget?.files?.[0]
          if (!file) return

          const text = await file.text()

          await importTheme(file.name, text)
          await refetch()
        }}
      />
    {/if}
    {#if tab === "shortcuts"}{/if}
  </div>
{/if}
