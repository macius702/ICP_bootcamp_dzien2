<template>
    <div>
        <h2 class="text-blue-600"> Wpisy na bloga</h2>
        <button @click="pobierzWpisy">refresh</button>
        <div v-for="wpis in wpisy">
            <p>{{wpis}}</p>
        </div>
        <input v-model="nowyBlog" type="text">
        <button @click="dodajWpisy">dodaj</button>
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
    async mounted(){
        this.pobierzWpisy()
    }
 
}
</script>