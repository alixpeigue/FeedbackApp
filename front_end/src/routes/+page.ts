export const load = async ({ fetch }) => {
    return {
        reports: await (async () => {
            const res = await fetch("http://localhost:3000/reports");
            return await res.json()
        })(),
        clients: await (async () => {
            const res = await fetch("http://localhost:3000/clients");
            return (await res.json()).map((el) => {return {value: ""+el.id, label: el.name}})
        })(),
        locations: await(async () => {
            const res = await fetch("http://localhost:3000/locations");
            return (await res.json()).map((el) => {return {value: ""+el.id, label: el.name}})
        })()
    }
}
