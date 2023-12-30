export const load = async ({ fetch }) => {
  return {
    reports: await (async () => {
      const res = await fetch("http://localhost:3000/reports");
      return await res.json()
    })()
  }
}
