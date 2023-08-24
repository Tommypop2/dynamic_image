import {
	ParentProps,
	children,
	createMemo,
	createSignal,
} from "solid-js";
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
				<DynamicImage>
					<div>{`This is extremely cool: ${count() * 2}`}</div>
				</DynamicImage>
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
