<script lang="ts">
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
<h1> Roulette</h1>
<h2>
    <p> You have {total} dollars</p>
    
    <input id="bet-amount" placeholder="Bet amount " bind:value="{bet}" />
    <input id="bet-amount" placeholder="Placement " bind:value="{placement}" />
    <button on:click="{play}">Place</button>
    <p>{text}</p>
    <p>{outcome}</p>
    <p>{winnings}</p>
</h2>