<script lang="ts">
  import { pushState } from "$app/navigation";
  import { page } from "$app/state";
  let currentPart: string = $derived(
    (page.state as { part?: string }).part ?? "intro",
  );
  let guide: {
    [key: string]: {
      title: string;
      content: string;
      next: { name: string; button: string }[];
    };
  } = {
    intro: {
      title: "Introduction",
      content: `Welcome to the Weather Index interactive setup guide! This guide will cover the following:
      <ul>
      <li>Installing r2modman</li>
      <li>Creating a profile</li>
      <li>Downloading Weather Index</li>
      <li>Configuring Weather Index</li>
      <li>Submitting runs</li>
      </ul>
      <p class="text-center"> Is this your first time modding Risk of Rain 2? </p>
      `,
      next: [
        { name: "r2modmanNotInstalled", button: "Yes" },
        { name: "creatingProfile", button: "No" },
      ],
    },
    r2modmanNotInstalled: {
      title: "r2modman",
      content: `Managing RoR2 mods is simple when using a mod manager such as r2modman or Thunderstore.
      <br>Download r2modman from <a href="https://thunderstore.io/c/riskofrain2/p/ebkr/r2modman/">this link</a> and continue once you're ready.
      <br><br>If you need any help, consult <a href="https://github.com/ebkr/r2modmanPlus#installing">the r2modman installation guide</a>.
      <p class="text-center">Proceed when r2modman is installed.</p>
      `,
      next: [{ name: "creatingProfile", button: "Proceed" }],
    },
    creatingProfile: {
      title: "Creating a profile",
      content: `
      <ol>
      <li>In r2modman, locate Risk of Rain 2 and click "Select Game"</li>
      <li> Select the profile you would like to add Weather Index to. <br>If this is your first time modding, use the "Default" profile</li>
      <li>Select the "Online" tab on the sidebar and search for "Weather Index"</li>
      <li> Select it and click "Download"</li>
      </ol>
      <video src="/guide/creatingProfile.webm" autoplay controls>
      <p class="text-center">Proceed when you are ready.</p>
      `,

      next: [{ name: "launchOptions", button: "Proceed" }],
    },
    launchOptions: {
      title: "Launch Options",
      content: `Would you like to always launch this modded profile when launching from Steam?
      <p class="text-center">This is purely for personal preference.</p>
      `,
      next: [
        { name: "setupR2modmanSteamArgs", button: "Yes" },
        { name: "launchNormally", button: "No" },
      ],
    },
    setupR2modmanSteamArgs: {
      title: "r2modman Steam Launching",
      content: `In r2modman, with the desired profile being active, do the following:
      <ol>
      <li>Select the "Help" tab on the sidebar</li>
      <li>Scroll down to the "Launching the game from outside the mod manager" section</li>
      <li>Click "Copy launch arguments"</li>
      <li>In Steam, right click Risk of Rain 2 and select "Properties"</li>
      <li>Paste the launch options under the "Launch Options" field.</li>
      </ol>
      Once this is done, launch the game from Steam and wait for it to load. If an extra window shows up, everything worked

      <p class="text-center">Proceed once the game has loaded.</p>
      `,

      next: [{ name: "configureWeatherIndex", button: "Proceed" }],
    },
    launchNormally: {
      title: "Launching",
      content: `In r2modman, with the desired profile being active, click "Start Modded" in the top left corner. <br>
      If an extra window shows up, everything worked

      <p class="text-center">Proceed once the game has loaded.</p>
      `,
      next: [{ name: "configureWeatherIndex", button: "Proceed" }],
    },
    configureWeatherIndex: {
      title: "Signing In",
      content: `In game, open Settings and navigate to the mod options.<br>
      Under the Weather Index category, click the "Connect" button to begin linking Risk of Rain 2 to Weather Index. <br>
      This will open a new tab in your web browser, with a prompt to connect RoR2. Approve it. <br><br>
       <b>You have to be signed in for this to work.</b> If you aren't signed in, sign in / sign up, navigate back to the page that was opened, and approve it.
      <video src="/guide/configureWeatherIndex.mp4" autoplay controls>
      <p class="text-center">Proceed once you get a popup confirming the connection.</p>
       `,
      next: [{ name: "submittingRuns", button: "Proceed" }],
    },
    submittingRuns: {
      title: "Submitting Runs",
      content: `Play through a run as normal. Once the game is over, there will be a "Submit" button. <br>If any kind of error occurs while submitting, double check the following and attempt to submit again:
      <ul>
      <li>Your internet connection. See if you can access this website (<a href="https://wi.shuflduf.xyz">wi.shuflduf.xyz</a>)</li>
      <li>Your authentication. You can pause the game on the end screen and navigate to the mod options for Weather Index. It should say "CONNECTED AS @[USERNAME]"</li>
      <li>Your mods. If you have any mods that add extra content to the game, it may not work.</li>
      </ul>
      You can only submit a run once.
      <p class="text-center">Congratulations on getting through the Weather Index guide!<br> I hope you enjoy what I made :D</p>
`,
      next: [],
    },
  };

  function changePart(next: string) {
    pushState("", {
      part: next,
    });
  }

  function goBack() {
    window.history.back();
  }
</script>

<div class="relative mx-auto w-xl">
  <h1 class="text-center text-5xl tracking-tighter">
    {guide[currentPart].title}
  </h1>
  {#if currentPart != "intro"}
    <button
      class="absolute top-2 cursor-pointer border bg-default p-2 transition-colors hover:bg-hover active:bg-hover"
      onclick={() => goBack()}
    >
      Go Back
    </button>
  {/if}
  <hr class="my-4" />
  <div
    class="prose text-lg text-primary marker:text-secondary prose-a:cursor-pointer prose-a:text-blue-500 prose-a:transition-colors prose-a:hover:text-blue-400 prose-a:active:text-blue-300"
  >
    {@html guide[currentPart].content}
  </div>
  <div class="mt-2 flex flex-row gap-2">
    {#each guide[currentPart].next as next}
      <button
        onclick={() => changePart(next.name)}
        class="w-full cursor-pointer border bg-default p-2 transition-colors hover:bg-hover active:bg-active"
      >
        {next.button}
      </button>
    {/each}
  </div>
</div>
