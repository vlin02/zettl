/** @type {import('tailwindcss').Config} */
export default {
  content: ["./src/**/*.{html,svelte,ts}"],
  theme: {
    colors: {
      primary: "var(--primary)",
      foreground: "var(--foreground)",
      background: "var(--background)",
      neutral: "var(--neutral)",
      "neutral-bg": "var(--neutral-bg)",
      transparent: "transparent",
      white: "white"
    }
  }
}
