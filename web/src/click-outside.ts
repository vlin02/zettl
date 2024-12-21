export function clickOutside(node: HTMLElement, fn: () => void) {
  const handleClick = (event: MouseEvent) => {
    if (!node.contains(event.target as Node)) {
      fn()
    }
  }

  document.addEventListener("click", handleClick, true)

  return {
    destroy() {
      document.removeEventListener("click", handleClick, true)
    }
  }
}
