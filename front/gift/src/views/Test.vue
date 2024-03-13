<template>
    <div ref="myDiv">Hello World</div>
</template>
   
  <script lang="ts">
  import { defineComponent } from 'vue';
   
  export default defineComponent({
    mounted() {
      // 获取 div 元素
      const myDiv = this.$refs.myDiv as HTMLDivElement;
      
      // 设置 CSS 样式
      myDiv.style.backgroundColor = "red";
      myDiv.style.color = "white";
      myDiv.style.padding = "10px";
    },
  });
  </script>