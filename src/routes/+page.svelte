<svelte:head>
    <title>Cole's Website</title>
    <meta name="description" content="Cole's Website">
</svelte:head>

<script>
    import Greet from './Login.svelte'
    import Background from './Background.svelte'
</script>

<Background/>
    <t1>
        <div class = "logo">
            <h1>Casino</h1>
        </div>
        <a href="/select" >
        <button class="button-5" style="display:block; margin: 0 auto;">
            
                Start
            
        </button>
    </a>
        
    </t1>
    

<style>
    .logo {
        color:#38c740;
        font-family: 'Segoe UI', Tahoma, Geneva, Verdana, sans-serif;
    }
    t1 {
        text-align: center;
        margin-top: auto;
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
    a {
        text-decoration: none;
        color: white;
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

