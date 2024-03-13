<template>
  <!--
    <div class="container">
      <h1>抽奖效果</h1>
      <button @click="startLottery" :disabled="isDisabled">开始抽奖</button>
      <div v-if="showResult" class="result">{{ result }}</div>
    </div>
    -->
  
  <div class="wrapper" >
    <div class="light"></div>
    <div class="light"></div>
    <div class="light"></div>
    <div class="light"></div>
    <div class="light"></div>
    <div class="light"></div>
    <div class="light"></div>
    <div class="light"></div>
    <div class="light"></div>
    <div class="light"></div>

    <div class="panel" >
      <div class="pointer"  @click="pointerClick" ref="pointer" @transitionend="pointerTransitionEnd">开始抽奖</div>
      <div class="sector">
        <div class="sector-inner">
          <span>谢谢参与</span>
        </div>
      </div>
      <div class="sector">
        <div class="sector-inner">
          <span>一等奖</span>
        </div>
      </div>
      <div class="sector">
        <div class="sector-inner">
          <span>谢谢参与</span>
        </div>
      </div>
      <div class="sector">
        <div class="sector-inner">
          <span>二等奖</span>
        </div>
      </div>
      <div class="sector">
        <div class="sector-inner">
          <span>谢谢参与</span>
        </div>
      </div>
      <div class="sector">
        <div class="sector-inner">
          <span>三等奖</span>
        </div>
      </div>
      <div class="sector">
        <div class="sector-inner">
          <span>谢谢参与</span>
        </div>
      </div>
      <div class="sector">
        <div class="sector-inner">
          <span>四等奖</span>
        </div>
      </div>
      <div class="sector">
        <div class="sector-inner">
          <span>谢谢参与</span>
        </div>
      </div>
      <div class="sector">
        <div class="sector-inner">
          <span>五等奖</span>
        </div>
      </div>
    </div>
  </div>
  <div ref="result"></div>
  <div @click="handleClick">点击我</div>
</template>

<script lang="ts">
import { collapseTextChangeRangesAcrossMultipleVersions } from 'typescript';
import { defineComponent, onMounted, ref } from 'vue'


function test() {
    console.log('clicke test!');
}

let onRotation = false;
let lights = ref([])       
let result:any ;
let pointer:any ;
let currentDeg = 0;

function doPointClick() { 
    console.log('pointer click')
    if (onRotation) return;
    console.log('开始抽奖');
    onRotation = true;
    lights.value.forEach(light => { light.className += ' light-twinkling'; });
    let nextStatus = getReward()();
    console.log(nextStatus)
    result.innerText = nextStatus.text;
    result.style.display = 'none';
    pointer.style.transform = `rotateZ(${nextStatus.deg}deg)`;
}


function getReward() {
    let reward = ['谢谢参与','一等奖','谢谢参与','二等奖','谢谢参与','三等奖','谢谢参与','四等奖','谢谢参与','五等奖']

    
    return function() {
        let ratateDeg = Math.random() * 360  + 1080;
        currentDeg += ratateDeg;
        let rewardText = reward[Math.floor((currentDeg + 18) % 360 / 36)]
        return {
            deg : currentDeg,
            text: rewardText == '谢谢参与' ? '很遗憾' : '恭喜：' + rewardText
        }
    }
}

function doPointerTransitionEnd() {
    console.log('抽奖结果');
    setTimeout(()=>{
        onRotation = false;
        lights.value.forEach(light => {light.className = 'light';});
        result.style.display = 'block';
    }, 300)
}

export default defineComponent({
  name: 'Raffe',
  methods : {
    pointerTransitionEnd() {
        doPointerTransitionEnd();
    }
  },
  mounted() {
      // 获取 div 元素
      result = this.$refs.result as HTMLDivElement;
      pointer = this.$refs.pointer as HTMLDivElement;
      lights.value = document.querySelectorAll('.light');
    
      // 设置 CSS 样式
      // result.style.backgroundColor = "red";
      // result.style.color = "white";
      // result.style.padding = "10px";
    },
  setup() {
    const showResult = ref(false)

    const lotteryRef = ref('lottery')



   


    function handleClick() {
      console.log('div 被点击了');
      test();
    }
    
    
    onMounted(() => {
        
        
        
        console.log('hello')
        console.log(result)

        
    })
    

    function pointerClick() {
        doPointClick();
    }

    return {
        lotteryRef,
        showResult,

        handleClick,
        pointerClick,
    }
  }
})
</script>

<style scoped>

* {
  margin: 0;
  padding: 0;
  border: none;
  outline: none;
  user-select: none;
}
.wrapper {
  position: relative;
  height: 240px;
  width: 240px;
  padding: 20px;
  margin: 20px;
  background-color: #c0381f;
  box-shadow: #000000 0px 0px 10px;
  border-radius: 50%;
}
.light {
  position: absolute;
  height: 10px;
  width: 10px;
  border-radius: 50%;
  top: 5px;
  left: 115px;
  transform-origin: 5px 115px;
}

.light-twinkling {
    animation: 1s twinkling 3, 100ms 3s twinkling 3;
}

.light:nth-child(2n) {
  background-color: #ffffff;
}
.light:nth-child(2n + 1) {
  background-color: #ade809;
}

.light:nth-child(2) {
  transform: rotate(36deg);
}
.light:nth-child(3) {
  transform: rotate(72deg);
}
.light:nth-child(4) {
  transform: rotate(108deg);
}
.light:nth-child(5) {
  transform: rotate(144deg);
}
.light:nth-child(6) {
  transform: rotate(180deg);
}
.light:nth-child(7) {
  transform: rotate(216deg);
}
.light:nth-child(8) {
  transform: rotate(252deg);
}
.light:nth-child(9) {
  transform: rotate(288deg);
}
.light:nth-child(10) {
  transform: rotate(324deg);
}
.panel {
  position: relative;
  height: 200px;
  width: 200px;
  background-color: #b7b7b7;
  border-radius: 100px;
}

.sector {
  position: absolute;
  left: 100px;
  top: 0px;
  width: 100px;
  height: 200px;
  font-size: 14px;
  border-radius: 0px 100px 100px 0;
  overflow: hidden;
  transform-origin: left center;
}
.sector:nth-child(2n) .sector-inner {
  background: #ffffff;
}
.sector:nth-child(2n + 1) .sector-inner {
  background: #ade809;
}
.sector:nth-child(2) {
  transform: rotate(-18deg);
}
.sector:nth-child(3) {
  transform: rotate(18deg);
}
.sector:nth-child(4) {
  transform: rotate(54deg);
}
.sector:nth-child(5) {
  transform: rotate(90deg);
}
.sector:nth-child(6) {
  transform: rotate(126deg);
}
.sector:nth-child(7) {
  transform: rotate(162deg);
}
.sector:nth-child(8) {
  transform: rotate(198deg);
}

.sector:nth-child(9) {
  transform: rotate(234deg);
}
.sector:nth-child(10) {
  transform: rotate(270deg);
}
.sector:nth-child(11) {
  transform: rotate(306deg);
}

.sector-inner {
  text-align: center;
  display: block;
  width: 100px;
  padding: 5px 3px 0 57px;
  height: 195px;
  background: #ffeab1;
  transform: translateX(-100px) rotate(36deg);
  transform-origin: right center;
  border-radius: 100px 0 0 100px;
}

.sector-inner span {
  display: block;
  transform-origin: center;
  transform: rotate(-18deg);
  color: #d46854;
}
.pointer {
  position: absolute;
  left: 79px;
  top: 79px;
  z-index: 10;
  height: 40px;
  width: 40px;
  padding: 6px;
  color: #fff899;
  line-height: 15px;
  font-size: 12px;
  text-align: center;
  background-color: #dc5b5b;
  border-radius: 50%;
  border: 1px solid #c0381f;
  transition: transform 3s cubic-bezier(0.2, 0.93, 0.43, 1);
}
.pointer::after {
  content: '';
  position: absolute;
  left: 14px;
  top: -24px;
  border-width: 12px 6px;
  border-style: solid;
  border-color: transparent;
  border-bottom-color: #c0381f;
}

@keyframes twinkling {
    50% { background: transparent; }
}

.result {
    margin: 20px 60px;
}
.test {
  width: 0;
  height: 0;
  border-style: solid;
  border-width: 100px 50px;
  border-color: red silver yellow purple;
}
</style>
