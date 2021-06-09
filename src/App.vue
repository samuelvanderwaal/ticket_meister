<template>
  <div id="app">
    <h1>Ticket Meister</h1>
    <br />
    <br />
    <div class="container">
      <div class="row">
        <div class="col"></div>
        <div class="col">
          <label for="private-key">Private Key</label>
        </div>
        <div class="col">
          <input
            v-model="privateKey"
            id="private-key"
            placeholder="private_key"
          />
        </div>
        <div class="col"></div>
      </div>
      <div class="row">
        <div class="col"></div>
        <div class="col">
          <label for="program-id">Program ID</label>
        </div>
        <div class="col">
          <input v-model="programId" id="program-id" placeholder="program_id" />
        </div>
        <div class="col"></div>
      </div>
      <div class="row">
        <div class="col"></div>
        <div class="col">
          <label for="max-tickets">Event Name</label>
        </div>
        <div class="col">
          <input
            v-model="eventName"
            id="event-name"
            placeholder="event_name (max 32 chars)"
          />
        </div>
        <div class="col"></div>
      </div>
      <div class="row">
        <div class="col"></div>
        <div class="col">
          <label for="max-tickets">Max Tickets</label>
        </div>
        <div class="col">
          <input
            v-model.number="maxTickets"
            id="max-tickets"
            placeholder="max_tickets"
          />
        </div>
        <div class="col"></div>
      </div>
      <br />
      <div>
        <div class="col"></div>
        <div class="col">
          <button @click="createEvent">Create Event</button>
        </div>
        <div class="col"></div>
      </div>
      <hr />
      <br /><br />
      <div class="row">
        <div class="col"></div>
        <div class="col">
          <label for="private-key">Private Key</label>
        </div>
        <div class="col">
          <input
            v-model="privateKey"
            id="private-key"
            placeholder="private_key"
          />
        </div>
        <div class="col"></div>
      </div>
      <div class="row">
        <div class="col"></div>
        <div class="col">
          <label for="program-id">Program ID</label>
        </div>
        <div class="col">
          <input v-model="programId" id="program-id" placeholder="program_id" />
        </div>
        <div class="col"></div>
      </div>
      <div class="row">
        <div class="col"></div>
        <div class="col">
          <label for="program-id">Event Account</label>
        </div>
        <div class="col">
          <input
            v-model="eventAccount"
            id="event-account"
            placeholder="event_account"
          />
        </div>
        <div class="col"></div>
      </div>
      <br />
      <div>
        <div class="col"></div>
        <div class="col">
          <button @click="purchaseTicket">Purchase Ticket</button>
        </div>
        <div class="col"></div>
      </div>
      <hr />
      <br /><br />
      <h3>Status</h3>
      <p>{{ status }}</p>
    </div>
  </div>
</template>

<script>
import { createEvent } from "./createEvent";
import { purchaseTicket } from "./purchaseTicket";

export default {
  name: "App",
  components: {},
  data() {
    return {
      ticketsPurchased: 0,
      maxTickets: 0,
      eventName: "",
      eventAccount: "",
      mintAccount: "",
      privateKey: "",
      programId: "Gd8xxsjfLh927PLth7TAC2CaBL5MsR8Vnm5waeXZGvW5",
      status: ""
    };
  },

  methods: {
    async createEvent() {
      let {
        eventAccountPubkey,
        isInitialized,
        initializerAccountPubkey,
        eventName,
        ticketsPurchased,
        maxTickets,
        mintAccount
      } = await createEvent(
        this.privateKey,
        this.eventName,
        this.maxTickets,
        this.programId
      );
      this.status = {
        eventAccountPubkey,
        isInitialized,
        initializerAccountPubkey,
        eventName,
        ticketsPurchased,
        maxTickets,
        mintAccount
      };
      console.log(
        eventAccountPubkey,
        isInitialized,
        initializerAccountPubkey,
        eventName,
        ticketsPurchased,
        maxTickets,
        mintAccount
      );
    },
    async purchaseTicket() {
      let {
        eventAccountPubkey,
        isInitialized,
        initializerAccountPubkey,
        ticketsPurchased,
        maxTickets
      } = await purchaseTicket(
        this.privateKey,
        this.eventAccount,
        this.programId
      );

      this.status = {
        eventAccountPubkey,
        isInitialized,
        initializerAccountPubkey,
        ticketsPurchased,
        maxTickets
      };
    }
  }
};
</script>

<style>
#app {
  font-family: Avenir, Helvetica, Arial, sans-serif;
  -webkit-font-smoothing: antialiased;
  -moz-osx-font-smoothing: grayscale;
  text-align: center;
  color: #2c3e50;
  margin-top: 60px;
}
</style>
