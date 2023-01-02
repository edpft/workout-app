export async function load({ fetch }) {
    const result = await fetch(`http://localhost:8000/workout/`);
    console.log(result);
    const item = await result.json();

    return { item };
}