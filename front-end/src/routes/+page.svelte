<script>
import { onMount } from "svelte";

let data = {};

onMount(async () => {
    fetch("http://localhost:8000/workout/")
    .then(response => response.json())
    .then(api_data => {
        console.log(api_data);
        data = api_data;
    }).catch(error => {
    console.log(error);
    return [];
    });
});
</script>

<main>
	<h1>{data.sport}</h1>
</main>