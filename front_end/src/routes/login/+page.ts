import type { PageLoad } from "../$types";

export const load: PageLoad = async ({ params, url }) => {
    let redirect = url.searchParams.get('redirected') == "true";
    return { redirect };
}
