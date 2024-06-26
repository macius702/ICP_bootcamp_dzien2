<template>
    <div>
        <h2 class="text-blue-600">Wpisy na bloga:</h2>
        <div class="w-100 flex flex-row-reverse">
            <button @click="pobierzWpisy" class="bg-blue-600 rounded text-white p-4">refresh</button>
        </div>
        <div class="grid mx-6 gap-4 my-4">
            <div v-for="(wpis, index) in wpisy" class="drop-shadow-xl bg-stone-300 p-4">
                <p>id: {{ index }}</p>
                <p v-if="editIndex !== index">{{ wpis }}</p>
                <input v-else v-model="editText" type="text">
                <button v-if="editIndex !== index" class="bg-blue-600 rounded text-white p-4" @click="deleteWpis(index)">usun</button>
                <button v-if="editIndex !== index" class="bg-green-600 rounded text-white p-4" @click="startEdit(index, wpis)">edit</button>
                <button v-else class="bg-green-600 rounded text-white p-4" @click="saveWpis">save</button>
            </div>
        </div>
        <div class="flex justify-center flex-col">
            <input v-model="nowyBlog" class="border-2 border-blue-600 p-4" type="text">
            <button  class="bg-blue-600 rounded text-white p-4" @click="dodajWpisy">dodaj</button>
        </div>
    </div>
</template>

<script>
import { dzien2_backend } from 'declarations/dzien2_backend/index';
export default {
    data() {
        return {
            wpisy: [],
            nowyBlog: '',
            editIndex: null,
            editText: ""            

        }
    },
    methods: {
        startEdit(index, wpis) {
            this.editIndex = index;
            this.editText = wpis;
        },
        async saveWpis() {
            await dzien2_backend.edit_text(this.editIndex, this.editText);
            this.editIndex = null;
            this.pobierzWpisy();
        },
        async dodajWpisy() {
            await dzien2_backend.add_text(this.nowyBlog);
            this.pobierzWpisy();
        },
        async deleteWpis(index) {
            await dzien2_backend.delete_text(index);
            await this.pobierzWpisy();
        },
        async pobierzWpisy() {
            let allTexts = await dzien2_backend.get_all_texts();
            console.log(allTexts);
            this.wpisy = allTexts;
        },
    },
    async mounted() {
        this.pobierzWpisy()
    }

}
</script>