import { createOpenGraphImage } from "@solid-mediakit/open-graph";
import server$ from "solid-start/server";
const DynamicImage = (props) => {
	const img = server$((r0) => {
		return createOpenGraphImage(<div>{r0}</div>);
	});
	const url = createMemo(() => {
		return img.url + `?args=${JSON.stringify(props.values)}`;
	});
	return <>{url()}</>;
};
import { ParentProps, children, createMemo, createSignal } from "solid-js";
import { Title } from "solid-start";
import Counter from "~/components/Counter";
const Image = (props: ParentProps) => {
	const kindern = children(() => props.children);
	const url = createMemo(() => kindern()?.toString());
	return <img src={url()}></img>;
};
export default function Home() {
	const [count, setCount] = createSignal(0);
	return (
		<main>
			<Title>Hello World</Title>

			<h1>Hello world!</h1>

			<Image>
				<DynamicImage values={[`This is extremely cool: ${count() * 2}`]} />
			</Image>

			<Counter count={count()} setCount={setCount} />

			<p>
				Visit{" "}
				<a href="https://start.solidjs.com" target="_blank">
					start.solidjs.com
				</a>{" "}
				to learn how to build SolidStart apps.
			</p>
		</main>
	);
}
