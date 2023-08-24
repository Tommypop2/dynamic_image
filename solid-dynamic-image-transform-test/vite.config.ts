import solid from "solid-start/vite";
import { PluginOption, defineConfig } from "vite";
import init, { transform } from "swc-plugin-dynamic-image";
import { readFile } from "fs/promises";
import { fileURLToPath } from "url";
function getExtension(filename: string): string {
	const index = filename.lastIndexOf(".");
	return index < 0 ? "" : filename.substring(index).replace(/\?.+$/, "");
}

const OpenGraphPlugin: PluginOption = {
	enforce: "pre",
	name: "opengraph",
	transform(code, id) {
		const currentFileExtension = getExtension(id);
		if (id.includes("node_modules")) {
			return;
		}
		if (id.includes("solid-start")) {
			return;
		}
		const extensionsToWatch = [".tsx", ".jsx"];
		if (!extensionsToWatch.includes(currentFileExtension)) {
			return null;
		}
		const out = transform(code, id);
		if (id.endsWith("root.tsx")) {
			// console.log(out);
		}
		return out;
	},
};
const wasmURL = new URL(
	"./node_modules/swc-plugin-dynamic-image/swc_plugin_dynamic_image_bg.wasm",
	import.meta.url
);
// console.log(wasmURL);
await init(await readFile(fileURLToPath(wasmURL)));
export default defineConfig({
	plugins: [OpenGraphPlugin, solid()],
});
