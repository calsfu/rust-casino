<script>
    import Background from '../Background.svelte'
	import Balance from '../Balance.svelte';
    let total = 500;
</script>

<Background/>

<button>
    <a href="/">
        Back
    </a>
</button>

<intro>
    <h1>Choose a game</h1>
    <Balance/>
</intro>


<button>
    <a href="/games/blackjack">
        <img src="src/lib/images/Casino-stock/blackjack.jpg" alt="Blackjack" width="300" height="200">
        <div class="caption">Blackjack</div>
    </a>
</button>
<button>
    <a href="/games/slots">
        <img src="src/lib/images/Casino-stock/slot-machines.jpg" alt="Blackjack" width="300" height="200">\
        <div class="caption">Slots</div>
    </a>
</button>
<button>
    <a href="/games/roulette">
        
        <img src="src/lib/images/Casino-stock/roulette.jpg" alt="Blackjack" width="300" height="200">
        <div class="caption">Roulette</div>
    </a>
</button>

<style>
    button {
        background-color: #38c740;
        cursor: pointer;
        font-family: system-ui,-apple-system,system-ui,"Helvetica Neue",Helvetica,Arial,sans-serif;
        font-size: 16px;
        font-weight: 600;
        justify-content: center;
    }
    button:hover {
        background-color: #187224;
    } 
    a {
        text-decoration: none;
        color: white;
    }
    intro {
        color:#38c740;
        font-family: 'Segoe UI', Tahoma, Geneva, Verdana, sans-serif;
    }
</style>