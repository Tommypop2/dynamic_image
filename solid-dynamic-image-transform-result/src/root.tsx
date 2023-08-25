import { createOpenGraphImage } from "@solid-mediakit/dynamic-image";
import server$ from "solid-start/server";
const DynamicImage = (props) => {
	const img = server$((r0) => {
		return createOpenGraphImage(<div style={r0}>Extra Cool Opengraph</div>);
	});
	const url = createMemo(() => {
		return img.url + `?args=${encodeURIComponent(JSON.stringify(props.values))}`;
	});
	return <>{url()}</>;
};
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
const OpenGraph = (
	props: ParentProps & {
		origin: string;
	}
) => {
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

				<OpenGraph origin="localhost:3001">
					<DynamicImage
						values={[
							{
								"font-size": "128px",
							},
						]}
					/>
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
