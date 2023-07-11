<script lang="ts">
	import Header from '../../Header.svelte';
    import { invoke } from '@tauri-apps/api/tauri'
    import { flip } from 'svelte/animate';
	import Background from '../../Background.svelte';
    import Balance from '../../Balance.svelte';
    let bet:number;
    let total = 500;
    let myCardsString = '';
    let dealerString = '';
    let endString = '';
    let gameStarted = false;
    let gameEnded = false;
    let playerCard1String = '';
    let playerCards:Array<string> = [];
    let dealerCards:Array<string> = [];
    var game:any;

    function update() {
        myCardsString = '';
        dealerString = '';
        playerCards = [];
        dealerCards = [];
        for(var card of game.table.players[0].hand) {
            let cardString = '/src/lib/images/' + card.suit + '/' + card.value  + '.png';
            playerCards.push(cardString);
        }
        for(var card of game.table.dealer.hand) {
            if(card.hidden) {
                dealerCards.push('/src/lib/images/PeterRiver.png');
                continue;
            }
            dealerCards.push('/src/lib/images/' + card.suit + '/' + card.value  + '.png');
        }
        playerCard1String = '/src/lib/images/' + game.table.dealer.hand[0].suit + '/' + game.table.dealer.hand[0].value  + '.png';
        let myCards = game.table.players[0].hand;
        let thierrCards = game.table.dealer.hand;
        for(var card of myCards) {
            myCardsString += '\n' + card.value + '\n ' + card.suit + ' ';
        }

        for(var card of thierrCards) {
            if(card.hidden) {
                dealerString += 'Hidden ';
            }
            else {
                dealerString += '\n' + card.value + '\n ' + card.suit + ' ';
            }
            
        }
        if(game.table.players[0].bust && !gameEnded) {
            end_game();
            myCardsString += '\nBust';
        }
        if(game.table.dealer.bust && !gameEnded) {
            end_game();
            dealerString += '\nBust';
        }
    }

    async function end_game() {
        gameEnded = true;
        game = await invoke('end_game', {game});
        update();
        if(game.table.players[0].bust) {
            myCardsString += '\nBust';
        }
        if(game.table.dealer.bust) {
            dealerString += '\nBust';
        }
        if(game.playerWin) {
            endString = 'You won ' + bet *2;
            total += bet * 2;
        }
        else if (game.table.players[0].blackjack_total == game.table.dealer.blackjack_total) {
            endString = 'Draw';
            total += bet;
        }
        else {
            endString = 'You lost';
        }

    }

    async function deal_card() {
        game = await invoke('deal_card', { game });
        update();
    }

    async function start_game() {
        if(bet > total) {
            alert('You do not have enough money');
            return;
        }
        total -= bet;
        gameEnded = false;
        endString = '';
        game = await invoke('start_blackjack', { bet: '1' });
        gameStarted = true;
        
        

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
    {#if gameStarted}
    <p>{endString}</p>
    <button class="button-5" on:click="{deal_card}">Deal</button>
    <button class="button-5" on:click="{end_game}">Hold</button>
   <p>My cards: </p>
    <!-- <img src="{playerCards[0]}" alt="Card 1" />
    <img src="{playerCards[1]}" alt="Card 2" />
    <img src="{playerCards[2]}" alt="Card 3" />
    <img src="{playerCards[3]}" alt="Card 4" />
    <img src="{playerCards[4]}" alt="Card 5" /> -->
    <cardimg>
        {#each playerCards as card}
            <img src="{card}" alt="Card 1" />
        {/each}
    </cardimg>
        

    <p>Dealer: </p>
    <!-- <img src="{dealerCards[0]}" alt="Card 6" />
    <img src="{dealerCards[1]}" alt="Card 7" /> -->
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