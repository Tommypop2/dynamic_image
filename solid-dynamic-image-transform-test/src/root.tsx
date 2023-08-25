// @refresh reload
import { ParentProps, Suspense, children, createMemo } from "solid-js";
import {
	A,
	Body,
	ErrorBoundary,
	FileRoutes,
	Head,
	Html,
	Meta,
	Routes,
	Scripts,
	Title,
} from "solid-start";
import "./root.css";
import { DynamicImage } from "@solid-mediakit/dynamic-image";
const OpenGraph = (props: ParentProps & { origin: string }) => {
	const kindern = children(() => props.children);
	const url = createMemo(() => kindern()?.toString());
	return <Meta property="og:image" content={props.origin + url()}></Meta>;
};
export default function Root() {
	return (
		<Html lang="en">
			<Head>
				<Title>SolidStart - Bare</Title>
				<Meta charset="utf-8" />
				<Meta name="viewport" content="width=device-width, initial-scale=1" />
				<OpenGraph origin="https://concerning-rush-arch-crisis.trycloudflare.com">
					<DynamicImage>
						<div style={{ "font-size": "128px" }}>Extra Cool Opengraph</div>
					</DynamicImage>
				</OpenGraph>
			</Head>
			<Body>
				<Suspense>
					<ErrorBoundary>
						<A href="/">Index</A>
						<A href="/about">About</A>
						<Routes>
							<FileRoutes />
						</Routes>
					</ErrorBoundary>
				</Suspense>
				<Scripts />
			</Body>
		</Html>
	);
}
