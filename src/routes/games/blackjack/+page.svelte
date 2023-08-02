<script lang="ts">
	import Header from '../../Header.svelte';
    import { invoke } from '@tauri-apps/api/tauri'
	import Background from '../../Background.svelte';
    import Balance from '../../Balance.svelte';
    import { balance } from '../../balance.js'
    let bet:number;
    let total:number;
    let myCardsString = '';
    let dealerString = '';
    let endString = '';
    let gameStarted = false;
    let gameEnded = false;
    let playerCards:Array<string> = [];
    let dealerCards:Array<string> = [];
    let game: BlackjackUpdate;
    let check:boolean = false; 
    type BlackjackUpdate = {
        player_cards: Array<Card>,
        dealer_cards: Array<Card>,
        game_over: boolean,
        player_win: boolean,
        no_money: boolean,
    }
    type Card = {
        suit: string,
        value: string,
        hidden: boolean,
    }
    async function start_game() {
   
        gameEnded = false;
        endString = '';
        game = await invoke('start_blackjack', { bet: bet });
        if(game.no_money) {
            endString = 'You do not have enough money';
            return;
        }
        check = game.no_money;
        gameStarted = true;
        let newBalance:number = await invoke('update_total', {});
        balance.set(newBalance);
        update();
    }

     function update() {
        myCardsString = '';
        dealerString = '';
        playerCards = [];
        dealerCards = [];
        // total = game.table.players[0].total;
        for(var card of game.player_cards) {
            let cardString = '/src/lib/images/' + card.suit + '/' + card.value  + '.png';
            myCardsString += '\n' + card.value + '\n ' + card.suit + ' ';
            playerCards.push(cardString);
        }
        for(var card of game.dealer_cards) {
            
            if(card.hidden) {
                dealerCards.push('/src/lib/images/PeterRiver.png');
                dealerString += 'Hidden ';
                continue;
            }
            dealerString += '\n' + card.value + '\n ' + card.suit + ' ';
            dealerCards.push('/src/lib/images/' + card.suit + '/' + card.value  + '.png');
        }
        if(game.game_over) {
            end_game();
        }
     
    }

    async function end_game() {
        gameEnded = true;
        game = await invoke('end_game', {game});
        if(game.player_win) {
            endString = 'You won ' + bet *2;
            total += bet * 2;
        }
        else {
            endString = 'You lost';
        }
        update();


        if(game.player_win) {
            endString = 'You won ' + bet *2;
            total += bet * 2;
        }
        else {
            endString = 'You lost';
        }
        let newBalance:number = await invoke('update_total', {});
        balance.set(newBalance);

    }

    async function deal_card() {
        game = await invoke('deal_card', {});
        update();
    }

    
 
</script>
<Header/>
<Background/>
<title>
    <h1> Blackjack</h1>
    <Balance/>
</title>

<h2>

    {#if !gameStarted || gameEnded}
    <input id="bet-amount" placeholder="Bet amount... " bind:value="{bet}" />
    <button class="button-5" on:click="{start_game}">Place</button>
    {/if}

    {#if gameStarted  }
    <p>{endString}</p>
    {#if !gameEnded}
    <button class="button-5" on:click="{deal_card}">Deal</button>
    <button class="button-5" on:click="{end_game}">Hold</button>
    {/if}
    <p>My cards: {myCardsString}</p>
    <cardimg>
        {#each playerCards as card}
            <img src="{card}" alt="Card 1" />
        {/each}
    </cardimg>

    <p>Dealer: {dealerString}</p>
    <cardimg>
        {#each dealerCards as card}
            <img src="{card}" alt="Card 1" />
        {/each}
    </cardimg>
    
    {/if}
</h2>

<style>
    h1 {
        text-align: center;
    }
    h2 {
        text-align: center;
    }
    p {
        text-align: center;
        color:#38c740;
    }
    cardimg {
        display: flex;
        justify-content: center;
    }
    img {
        width: 70px;
        height: auto;
        margin: 10px;
    }
    title {
        display: grid;
        justify-content: center;
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

    input {
        height: 30px;
        border-radius: 10px;
        border : 1px solid #38c740;
    }
</style>