<script lang="ts">
  import { invoke } from "@tauri-apps/api/tauri"

  let name = "";
  let greetMsg: string[] = []

  async function greet(a){
    console.log(a);
    
    const lines: string[] = await Promise.all(name.split("\n").map(async t => await invoke("greet", {name: t, separator: "_"})))

    greetMsg = lines;
    
  }
</script>

<div>
  <div class="row">
    <textarea id="greet-input" 
    bind:value={name}
    on:input={greet} placeholder="Enter a name..."  
    style="width:700px; height: 300px"
    />
  </div>
  <p>
    {#each greetMsg as line}
      {@html line}<br />
    {/each}</p>
</div>