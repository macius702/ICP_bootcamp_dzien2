<script setup>
import { ref, onMounted } from 'vue';
import { dzien2_backend } from 'declarations/dzien2_backend/index';
let greeting = ref('');
let allTexts = ref([]);

async function handleAddText(e) {
  e.preventDefault();
  const target = e.target;
  const text = target.querySelector('#text').value;
  await dzien2_backend.add_text(text);
  await handleGetAllTexts(); // Fetch the updated list of all texts
}

async function handleGetAllTexts() {
  await dzien2_backend.get_all_texts().then((response) => {
    allTexts.value = response;
  });
}
onMounted(handleGetAllTexts); // Fetch all texts right after loading the page
</script>

<template>
  <main>
    <img src="/logo2.svg" alt="DFINITY logo" />
    <br />
    <br />

    <form action="#" @submit="handleAddText">
      <label for="text">Dodaj_wpis: &nbsp;</label>
      <input id="text" alt="Text" type="text" />
      <button type="submit">Add Text</button>
    </form>
    <button @click="handleGetAllTexts">Get All Texts</button>
    <section id="allTexts">
      <ul>
        <li v-for="(text, index) in allTexts" :key="index">{{ text }}</li>
      </ul>
    </section>
  </main>
</template>