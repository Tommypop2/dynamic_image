import { createSignal, createMemo, Show } from "solid-js";
import { Title } from "solid-start";
import Counter from "~/components/Counter";
import { DynamicImage, Image } from "@solid-mediakit/dynamic-image";
import { ParentProps } from "solid-js/types/render";
const Container = (props: ParentProps) => {
	return <div style={{ height: "200px" }}>{props.children}</div>;
};
export default function Home() {
	const [count, setCount] = createSignal(0);
	const [show, setShow] = createSignal(true);
	return (
		<main>
			<Title>Hello World</Title>
			<h1>Hello world!</h1>
			<button onClick={() => setShow((p) => !p)}>Show/Hide</button>
			<div style={{ display: "flex", "flex-direction": "column" }}>
				<Show when={show()}>
					<Container>
						<Image>
							<DynamicImage>
								<div>{`This is extremely cool: ${count() * 2}`}</div>
							</DynamicImage>
						</Image>
					</Container>
				</Show>
				<Container>
					<Image>
						<DynamicImage>
							<div>{`This is also extremely cool: ${count() * 3}`}</div>
						</DynamicImage>
					</Image>
				</Container>
				<Container>
					<Image>
						<DynamicImage>
							{async () => {
								type User = { name: { first: string } };
								const res = await fetch("https://randomuser.me/api/");
								const users: { results: User[] } = await res.json();
								return (
									<div
										style={{
											width: "100%",
											height: "100%",
											display: "flex",
											"font-size": "54px",
											background: "lavender",
										}}
									>
										{`Hello ${users.results[0].name.first}!`}
									</div>
								);
							}}
						</DynamicImage>
					</Image>
				</Container>
			</div>
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
