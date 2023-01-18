<script lang="ts">
  import { save } from "@tauri-apps/api/dialog"
  import { invoke } from "@tauri-apps/api/tauri"
  import { output } from "./output";
  import type { Output } from "../type";

  //props
  export let back:()=>void;

  //get output
  let out:Output
  output.subscribe(e=> out=e)
  $: inspected = out.status==200 ? true : true;
  $: machine_items = out.machine.map(i=>({id:i,name:i}))
  let greetMsg = ""
  let status = false
  let please_select:string
  async function greet(){
    const file = await save({filters: [{extensions: ["csv"],name: "data"}]});
    if (file==null){
      greetMsg = "No file selected"
    }else{
      let res = await invoke("get_result",{path:file,machine:please_select}) as boolean
      status = res
      inspected = false
    }
  }
</script>

<div class="full_sc">
  {#if !inspected}
  <h1>{!status?"There is Error":"Sucesfully Get the Output"}</h1>
  <button class="inspect" on:click={back}>
    Back
  </button>
  {:else}
  <button class="inspect" on:click={back}>
    Back
  </button>
    <div class="stat">
      <h1>Data Telah Diproses Keseluruhan</h1>
      <h3>{`Total data yang diproses = ${out.data}`}</h3>
      <h3>{`Total data invalid (tidak masuk waktu sholat) = ${out.invalid}`}</h3>
      <h3>{`Total data double (ceklok 2x atau lebih) = ${out.double}`}</h3>
      <h3>{`Total data valid (masuk waktu sholat) = ${out.valid}`}</h3>
    </div>
  <p class="status">Pilih mesin yang akan diambil datanya</p>
  <div class="nice-col">
    <select bind:value={please_select} on:change={()=>greetMsg=please_select}>
      {#each machine_items as item}
        <option value={item.id}>{item.name}</option>
      {/each}
    </select>
    <div class="nice-row">
    <p>Start</p>
    <input type="date" bind:value={out.start}>
    </div>
    <div class="nice-row">
    <p>Stop</p>
    <input type="date" bind:value={out.finish}>
    </div>
    <button class="inspect" on:click={greet}>
      Get Data
    </button>
  </div>
  {/if}
  <p class="status-idk">{greetMsg}</p>
</div>
<style>
  .inspect{
    font-size: 1.5rem;
    padding: 0.5rem;
  }
  .status{
    font-size: 1.5rem;
  }
  .status-idk{
    position: absolute;
    top: 20vh;
    font-size:1.5rem;
  }
  .nice-col{
    display: flex;
    flex-direction: column;
    justify-content: center;
    align-items: center;
  }
  .nice-row{
    display: flex;
    align-items: center;
    gap: 0.5rem;
  }
  .stat{
    border: solid 2px;
    border-color: #aaa;
    padding: 2rem;
    border-radius: 20%;
    margin-top: 1rem;
  }
  .stat h3{
    padding: 0;
    margin: 0;
  }
</style>
