<template>
    <div>
        <h2 class="text-blue-600"> Wpisy na bloga</h2>
        <div class="w-100 flex flex-row-reverse">
            <button @click="pobierzWpisy" class="bg-blue-600 rounded text-white 
            p-4">refresh</button>
        </div>

        <div class="grid mx-6 gap-4 my-4">
            <div v-for="wpis in wpisy" class="drop-shadow-x1 bg-stone-300 p-4">
                <p>{{ wpis }}</p>
            </div>
        </div>
        <div class="flex justify-center flex-col">
            <input v-model="nowyBlog" type="text">
            <button @click="dodajWpisy" class="bg-blue-600 rounded-sm text-white p-4">dodaj</button>
        </div>
    </div>
</template>

<script>
import { dzien2_backend } from 'declarations/dzien2_backend/index';
export default {
    data() {
        return {
            wpisy: [],
            nowyBlog: ''

        }
    },
    methods: {
        async pobierzWpisy() {
            let allTexts = await dzien2_backend.get_all_texts();
            console.log(allTexts);
            this.wpisy = allTexts;
        },
        async dodajWpisy() {
            await dzien2_backend.add_text(this.nowyBlog);
            this.pobierzWpisy();
        }

    },
    async mounted() {
        this.pobierzWpisy()
    }

}
</script>