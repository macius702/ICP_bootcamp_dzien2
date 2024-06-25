<template>
    <div>
        <button @click="pobierzWpisy">refresh</button>
        siema blog!!
        <input v-model="nowyBlog" type="text">
        <button @click="dodajWpisy">dodaj</button>
        {{ wpisy }}
    </div>
</template>

<script>
import { dzien2_backend } from 'declarations/dzien2_backend/index';
export default {
    data() {
        return {
            wpisy: []
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
 
}
</script>