<svelte:head>
    <title>Cole's Website</title>
    <meta name="description" content="Cole's Website">
</svelte:head>

<script>
    import Greet from './Login.svelte'
</script>

<t1>
    <h2>Casino</h2>
    <button style="display:block; margin: 0 auto;">
        <a href="/select">
            Start
        </a>
    </button>
</t1>

  <h1>Welcome to SvelteKit</h1>
  <Greet />

<style>
    t1 {
        text-align: center;
        margin-top: auto;
    }
</style>