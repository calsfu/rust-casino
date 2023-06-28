<script lang="ts">
	import Background from '../../Background.svelte';
    import Header from '../../Header.svelte';
    import { invoke } from '@tauri-apps/api/tauri'
    type Roulette = {
        number: number,
        color: String
        net: number,
    }
    let bet = 0;
    let placement = 'B';
    let total = 500;
    let text = '';
    let outcome = '';
    let winnings = '';
    async function play() {

        if(bet < 0) {
            text = "You can't place a negative bet!";
            return;
        }
        if(bet > total) {
            text = "You don't have enough money to place that bet!";
            return;
        }
        text = "You placed " + bet + " dollars on " + placement;
        total -= bet;
        let game:Roulette = await invoke('play_roulette', {
            bets: bet, 
            placement: placement,
            });
        outcome = "The ball landed on " +  game.number + " which is " + game.color;
        winnings = "You won " + game.net + " dollars!";
        total += game.net;
    }
    // async function check_bet() {
    //     temp = await invoke('get_roll', { bet })
    // }
</script>
<Header/>
<Background/>
<title>
    <h1> Roulette</h1>
    <h2> You have {total} dollars</h2>
</title>

<h2>
    
    
    <input id="bet-amount" placeholder="Bet amount " bind:value="{bet}" />
    <input id="bet-amount" placeholder="Placement " bind:value="{placement}" />
    <button class="button-5" on:click="{play}">Place</button>
    <p>{text}</p>
    <p>{outcome}</p>
    <p>{winnings}</p>
</h2>

<style>
    input {
        height: 30px;
        border-radius: 10px;
        border : 1px solid #38c740;
    }
    p {
        color: #38c740;
        font-family: 'Segoe UI', Tahoma, Geneva, Verdana, sans-serif;
    }
    title {
        display: grid;
        color:#38c740;
        font-family: 'Segoe UI', Tahoma, Geneva, Verdana, sans-serif;
    }
    .button-5 {
    align-items: center;
    background-clip: padding-box;
    background-color: #38c740;
    border: 1px solid transparent;
    border-radius: .25rem;
    box-shadow: rgba(0, 0, 0, 0.02) 0 1px 3px 0;
    box-sizing: border-box;
    color: #fff;
    cursor: pointer;
    display: inline-flex;
    font-family: system-ui,-apple-system,system-ui,"Helvetica Neue",Helvetica,Arial,sans-serif;
    font-size: 16px;
    font-weight: 600;
    justify-content: center;
    line-height: 1.25;
    margin: 0;
    min-height: 3rem;
    padding: calc(.875rem - 1px) calc(1.5rem - 1px);
    position: relative;
    text-decoration: none;
    transition: all 250ms;
    user-select: none;
    -webkit-user-select: none;
    touch-action: manipulation;
    vertical-align: baseline;
    width: auto;
    }
    .button-5:hover,
    .button-5:focus {
    background-color: #187224;
    box-shadow: rgba(0, 0, 0, 0.1) 0 4px 12px;
    }

    .button-5:hover {
    transform: translateY(-1px);
    }

    .button-5:active {
    background-color: #187224;
    box-shadow: rgba(0, 0, 0, .06) 0 2px 4px;
    transform: translateY(0);
    }
</style>