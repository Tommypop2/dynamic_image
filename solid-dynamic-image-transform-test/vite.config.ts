import solid from "solid-start/vite";
import { Plugin, defineConfig } from "vite";
import { vitePlugin } from "@solid-mediakit/dynamic-image/unplugin";

export default defineConfig({
	plugins: [vitePlugin() as Plugin, solid()],
});
