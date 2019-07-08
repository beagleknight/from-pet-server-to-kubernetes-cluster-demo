<template>
  <form @submit="say($event)">
    <h1>Say my name!</h1>
    <input v-model="name" />
    <input type="submit" value="Say it!" />
    <p>{{ result }}</p>
  </form>
</template>

<script>
import HelloWorld from "./components/HelloWorld.vue";

export default {
  name: "app",
  components: {
    HelloWorld
  },
  data() {
    return {
      name: "David",
      result: ""
    };
  },
  methods: {
    say(event) {
      fetch(`http://localhost:8000/${this.name}`)
        .then(res => res.text())
        .then(res => (this.result = res));
      event.preventDefault();
    }
  }
};
</script>

<style>
#app {
  font-family: "Avenir", Helvetica, Arial, sans-serif;
  -webkit-font-smoothing: antialiased;
  -moz-osx-font-smoothing: grayscale;
  text-align: center;
  color: #2c3e50;
  margin-top: 60px;
}
</style>
