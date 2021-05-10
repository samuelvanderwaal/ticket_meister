<template>
  <div id="app">
    <h1>Ticket Meister</h1>
    <br />
    <br />
    <b-container>
      <b-row>
        <b-col></b-col>
        <b-col>
          <label for="private-key">Private Key</label>
        </b-col>
        <b-col>
          <b-form-input
            id="private-key"
            placeholder="private_key"
          ></b-form-input>
        </b-col>
        <b-col> </b-col>
      </b-row>
      <b-row>
        <b-col></b-col>
        <b-col>
          <label for="program-id">Program ID</label>
        </b-col>
        <b-col>
          <b-form-input id="program-id" placeholder="program_id"></b-form-input>
        </b-col>
        <b-col> </b-col>
      </b-row>
      <b-row>
        <b-col></b-col>
        <b-col>
          <label for="max-tickets">Max Tickets</label>
        </b-col>
        <b-col>
          <b-form-input
            id="max-tickets"
            placeholder="max_tickets"
          ></b-form-input>
        </b-col>
        <b-col></b-col>
      </b-row>
      <br />
      <b-row>
        <b-col></b-col>
        <b-col>
          <b-button @click="createEvent">Create Event</b-button>
        </b-col>
        <b-col></b-col>
      </b-row>
      <hr />
      <br /><br />
      <h3>Status</h3>
      <p>{{ status }}</p>
    </b-container>
  </div>
</template>

<script>
import { createEvent } from "./createEvent";

export default {
  name: "App",
  components: {},
  data() {
    return {
      status: ""
    };
  },

  methods: {
    createEvent() {
      console.log("creating event!");
    },
    buyTicket() {
      console.log("buying ticket!");
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
