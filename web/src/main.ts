import { mount } from 'svelte'
import App from './counter.svelte'
import "./app.css";

const app = mount(App, {
  target: document.getElementById('app')!,
})

export default app
