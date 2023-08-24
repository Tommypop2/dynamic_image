import { createOpenGraphImage } from "@solid-mediakit/open-graph";
import server$ from "solid-start/server";
const DynamicImage = (props)=>{
    const img = server$(()=>{
        return createOpenGraphImage(<div>123</div>);
    });
    const url = createMemo(()=>{
        return img.url + `?args=${JSON.stringify(props.values)}`;
    });
    return <>{url()}</>;
};
const coolVar = <DynamicImage values={[]}/>;
