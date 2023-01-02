<script lang="ts">
    import type { PageData } from './$types';
    import Cross from '$lib/Cross.svelte';
    import Step from '$lib/Step.svelte';

    export let data: PageData;
</script>

<header class="l-row | c-header">
    <button class="c-header__close-button">
        <Cross />
    </button>
    <h1 class="c-header__title">{data.name}</h1>
    <button class="c-header__save-button">save</button>
</header>
<main>
    <section>
        <h2>Overview</h2>
        <section>
            <h3>Notes</h3>
            <p>{data.notes}</p>
        </section>
    </section>
    <section>
        <h2>Steps</h2>
        <ul>
            {#each data.steps as step}
            <li>
                <Step type={step.type} duration={step.duration} />
            </li>
            {/each}
        </ul>
    </section>
    <div>
        <button>Add step</button>
        <button>Add repeat</button>
    </div>
</main>

<style lang="scss">
    *,*::before,*::after {
        box-sizing: border-box;
    }

    .l-row {
        display: flex;
    }

    .c-header {
        &__close-button {
            padding: 0.5rem;
        }
        &__save-button {
            text-transform: uppercase;
        }
    }
</style>