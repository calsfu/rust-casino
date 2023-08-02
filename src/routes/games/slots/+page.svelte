<script lang="ts">
    import Background from '../../Background.svelte';
    import Header from '../../Header.svelte';
    import Balance from '../../Balance.svelte';
    import { invoke } from '@tauri-apps/api/tauri'
	import { balance } from '../../balance';
    type Slots = {
        total : number, 
        slot1: number, 
        slot2: number,
        slot3: number,
        outcome: string,
    }
    let place: number;
    let text = '';
    let s1 = "/src/lib/images/Slot/7.jpg";
    let s2 = "/src/lib/images/Slot/7.jpg";
    let s3 = "/src/lib/images/Slot/7.jpg";
    let options:string[] = ["/src/lib/images/Slot/7.jpg", "/src/lib/images/Slot/diamond.jpg", "/src/lib/images/Slot/cherries.jpg"];
    async function play() {
        //0 is 74
        //1 is diamond
        //2 is cherry
        let bet = place;
        let game:Slots = await invoke('play_slots', {
            bets: bet, 
        });
        s1 = options[game.slot1];
        s2 = options[game.slot2];
        s3 = options[game.slot3];
        balance.set(game.total);
        //total += game.net;
        text = game.outcome;
        
    }
</script>
<Header/>
<Background/>
<title>
    <h1> Slots</h1>
    <Balance/>
    <h3>{text}</h3>
</title>

<input id="bet-amount" placeholder="Bet amount " bind:value="{place}" />
<button class="button-5" on:click="{play}">Place</button>
<machine>
    <img class="slot" alt="slot1" id="one" src={s1}>
    <img class="slot" alt="slot2" id="two" src={s2}>
    <img class="slot" alt="slot3" id="three"src={s3}>   
</machine>



<style>
    machine {
        display: flex;
       margin-left: 40%;
    }
    .slot {
        width: 100px;
        height: 100px;
        border-radius: 10px;
        border : 1px solid #38c740;
    }
    input {
        height: 30px;
        border-radius: 10px;
        border : 1px solid #38c740;
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