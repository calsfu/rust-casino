<script lang='ts'>
    import { invoke } from '@tauri-apps/api/tauri'
    import { balance } from './balance.js'

    let balVal: number;
    balance.subscribe(value => {
        balVal = value;
    })
</script>

<h2>Balance: {balVal}</h2>

<style>
    h2 {
        color:#38c740;
        font-family: 'Segoe UI', Tahoma, Geneva, Verdana, sans-serif;
    }
</style>