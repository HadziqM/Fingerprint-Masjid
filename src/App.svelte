<script lang="ts">
  import { open } from "@tauri-apps/api/dialog"
  import { invoke } from "@tauri-apps/api/tauri"
  import { output } from "./lib/output";
  import Greet from "./lib/Greet.svelte";
  import type { Output } from "./type";


  let greetMsg = ""
  let loadingScreen = false
  let main = true

  let d_s = "11:00"
  let d_f ="12:00"
  let a_s = "14:30"
  let a_f ="15:30" 
  let m_s = "17:10"
  let m_f ="18:10" 
  let i_s = "18:40"
  let i_f ="19:40" 
  let s_s = "03:20"
  let s_f ="04:10" 
  let t_s = "02:00"
  let t_f ="03:19"
  
  async function file_parse(path:string,csv:boolean){
      const all:string = [d_s,d_f, a_s,a_f,m_s,m_f, i_s,i_f,s_s,s_f,t_s,t_f].join(",");
      const res = await invoke("greet",{path,all,csv}) as string
      output.set(JSON.parse(res) as Output);
      greetMsg = ""
      loadingScreen = false
      main = false
  }

  let get_path = async (csv:boolean) =>{
    const extension = csv?"csv":"html"
    const file = await open({filters:[{extensions:[extension],name:"data"}]})
    if (file instanceof Array || file == null){
      greetMsg = "No file selected"
      loadingScreen = false
    }else{
      loadingScreen = true
      greetMsg = "loading ....."
      await file_parse(file,csv)
   }
  }
</script>

{#if !main}
  <Greet back={()=>main=true}/>
{:else}
  {#if loadingScreen}
    <div class="full_sc">
      <h1>loading.....</h1>
    </div>
  {:else}
<div class="full_sc">
    <h2>{greetMsg}</h2>
  <a href="https://www.convertcsv.com/html-table-to-csv.htm" target="_blank">Click This to convert Html to CSV</a>
  <form class="form-s">
    <div class="bg-container">
      <div class="container">
        <p>Duhur</p>
        <div class="f-row">
        <div class="f-col"><label for="d_s">Start</label>
        <input type=time name="d_s" bind:value={d_s} ></div>
        <div class="f-col"><label for="d_f">Stop</label>
        <input type=time name="d_f" bind:value={d_f}></div>
        </div>
      </div>
      <div class="container">
        <p>Asyar</p>
        <div class="f-row">
        <div class="f-col"><label for="a_s">Start</label>
        <input type=time name="a_s" bind:value={a_s} ></div>
        <div class="f-col"><label for="a_f">Stop</label>
        <input type=time name="a_f" bind:value={a_f}></div>
        </div>
      </div>
      <div class="container">
        <p>Maghrib</p>
        <div class="f-row">
        <div class="f-col"><label for="m_s">Start</label>
        <input type=time name="m_s" bind:value={m_s} ></div>
        <div class="f-col"><label for="m_f">Stop</label>
        <input type=time name="m_f" bind:value={m_f}></div>
        </div>
      </div>
      <div class="container">
        <p>Isya'</p>
        <div class="f-row">
        <div class="f-col"><label for="d_s">Start</label>
        <input type=time name="i_s" bind:value={i_s} ></div>
        <div class="f-col"><label for="i_f">Stop</label>
        <input type=time name="i_f" bind:value={i_f}></div>
        </div>
      </div> 
      <div class="container">
        <p>Subuh</p>
        <div class="f-row">
        <div class="f-col"><label for="s_s">Start</label>
        <input type=time name="d_s" bind:value={s_s} ></div>
        <div class="f-col"><label for="s_f">Stop</label>
        <input type=time name="s_f" bind:value={s_f}></div>
        </div>
      </div>
      <div class="container">
        <p>Tahajud</p>
        <div class="f-row">
        <div class="f-col"><label for="t_s">Start</label>
        <input type=time name="t_s" bind:value={t_s} ></div>
        <div class="f-col"><label for="t_f">Stop</label>
        <input type=time name="t_f" bind:value={t_f}></div>
        </div>
      </div> 
    </div>
    <div class="button-holder">
    <button on:click|preventDefault={() => {
      loadingScreen = true
      get_path(true)
    }}>
      Proses CSV
    </button>
    <button on:click|preventDefault={() => {
      loadingScreen = true
      get_path(false)
    }}>
      Proses HTML
    </button>
    </div>
  </form>
</div>
  {/if}
{/if}

<style>
  .button-holder{
    display: flex;
    justify-content: center;
    align-items: center;
    gap: 1rem;
  }
  .button-holder button{
    font-size: 1.5rem;
  }
  .form-s{
    display: flex;
    flex-direction: column;
    padding: 1rem;
    border: solid #eee;
    border-radius: 10%;
  }
  .bg-container{
    display: grid;
    grid-template-columns: auto auto;
    padding: 0.5rem;
    gap: 0.3rem;
  }
  .container{
    display: flex;
    flex-direction: column;
    align-items: center;
    justify-content: center;
    border: solid #aaa;
    border-radius: 10%;
    padding: 0.5rem;
  }
  .container p{
    width: 100%;
    background-color: #999;
    color: #333;
    text-align: center;
    font-size: 1rem;
    font-weight: bold;
  }
  .container label{
    width: 100%;
    text-align: center;
    font-size: 0.7rem;
  }
  .f-row{
    display: flex;
    gap: 1rem;
  }
  .f-col{
    display: flex;
    gap: 0.1rem;
    flex-direction: column;
  }
</style>
