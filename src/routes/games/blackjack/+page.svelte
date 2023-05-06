<script lang="ts">
    import { invoke } from '@tauri-apps/api/tauri'
    let bet = '';
    let myCardsString = '';
    let dealerString = '';
    let gameStarted = false;
    var game:any;

    function update() {
        myCardsString = '';
        dealerString = '';
        let myCards = game.table.players[0].hand;
        let dealerCards = game.table.dealer.hand;
        for(var card of myCards) {
            myCardsString += '\n' + card.value + '\n ' + card.suit + ' ';
        }

        for(var card of dealerCards) {
            if(card.hidden) {
                dealerString += 'Hidden ';
            }
            else {
                dealerString += '\n' + card.value + '\n ' + card.suit + ' ';
            }
            
        }
    }

    async function deal_card() {
        game = await invoke('deal_card', { game });
        update();
    }

    async function start_game() {
        game = await invoke('start_blackjack', { bet: '1' });
        gameStarted = true;
        
        

        update();
    }
 
</script>
<h1> Blackjack</h1>
<h2>
    <input id="bet-amount" placeholder="Bet amount " bind:value="{bet}" />
    <button on:click="{start_game}">Place</button>
   <p>My cards: {myCardsString}</p>
    <p>Dealer: {dealerString}</p>
    <button on:click="{deal_card}">Deal</button>
</h2>