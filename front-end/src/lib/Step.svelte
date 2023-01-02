<script lang="ts">
    export let type: string
    export let duration: number[];
</script>

<section>
    <h3>{type}</h3>
    <p>{duration}</p>
</section>
<button>Move</button>
<button>Delete</button>
