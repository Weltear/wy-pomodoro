<script setup>
import { Window } from "@tauri-apps/api/window"
import { Tools } from "@element-plus/icons-vue";
import { invoke } from "@tauri-apps/api/core";
import { onMounted, ref } from "vue";
import { config } from "../main";

// 倒计时变量
const timeRemaining = ref(0);
const workTime = ref(0.5);
const breakTime = ref(0.5);
// 当前状态
const mode = ref('work');
// 提示词
const note = ref("工作剩余时间");
// 程序运行状态
const running = ref(false);
// 设置界面状态
const settingDrawer = ref(false);

const homeWindow = new Window('homeWindow')

timeRemaining.value = getValue(workTime);


// 程序开始
function begin() {
    running.value = true;
}

function stop() {
    running.value = false;
    mode.value = 'work';
    note.value = "工作剩余时间";
    timeReset();
}

// 获取倒计时
function getValue(time) {
    return Date.now() + time * 1000 * 60;
}

// 倒计时重置
function timeReset() {
    if (mode.value === 'work') {
        timeRemaining.value = getValue(workTime.value);
    } else {
        timeRemaining.value = getValue(breakTime.value);
    }
}

// 时间变动
function timeChange() {
    if (!running.value) {
        timeRemaining.value = getValue(workTime.value);
    }
}

// 倒计时结束
async function timeFinish() {
    // 当前工作，进入休息
    if (running.value) {
        if (mode.value === 'work') {
            mode.value = 'rest';
            note.value = "休息剩余时间";
            timeRemaining.value = getValue(breakTime.value);
            // 页面弹回
            // homeWindow.show();
            // 锁屏
            await invoke('lock_screen', {window: homeWindow});
        } else {
            // 否则，进入工作
            mode.value = 'work';
            note.value = "工作剩余时间";
            timeRemaining.value = getValue(workTime.value);
        }
    }
    
}

async function getConfig() {
    let configString = await invoke('get_config');
    config.value = JSON.parse(configString);

    console.log("读取设置", config.value);

    configUpdate();
}

async function saveConfig() {
    config.value.work_time = workTime.value;
    config.value.rest_time = breakTime.value;
    console.log("设置保存", config.value);
    let configString = JSON.stringify(config.value);
    await invoke('save_config', {config: configString});
}

// 时间设置更新函数
function configUpdate() {
    workTime.value = config.value.work_time;
    breakTime.value = config.value.rest_time;
}

// 页面启动钩子
onMounted(async () => {
    await getConfig();
})
</script>

<template>
    <div class="home-view">
        <div class="header">番茄钟</div>

        <!-- 倒计时区域 -->
         <el-countdown
            :title="note"
            class="time-count"
            format="HH:mm:ss"
            :value="timeRemaining"
            @change="timeChange"
            @finish="timeFinish" />

        <div v-if="!running">
            <el-button type="primary" @click="begin">开始</el-button>
        </div>

        <div v-else>
            <el-button type="primary" @click="timeReset">重置</el-button>
            <el-button type="primary" @click="stop">终止</el-button>
        </div>

        <div class="setting">
            <el-button type="info" :icon="Tools" size="large" @click="settingDrawer=true" circle/>
        </div>

        <!-- 设置抽屉 -->
         <el-drawer
            v-model="settingDrawer"
            title="设置"
            :size="'50%'"
            :direction="'btt'"
            @close="saveConfig">
            <div class="work-setting">
                <span style="margin-right: 10px;">工作时间</span>
                <el-input-number v-model="workTime" :precision="2" :controls="false">
                    <template #suffix>
                        <span>分钟</span>
                    </template>
                </el-input-number>
            </div>

            <div class="rest-setting">
                <span style="margin-right: 10px;">休息时间</span>
                <el-input-number v-model="breakTime" :precision="2" :controls="false">
                    <template #suffix>
                        <span>分钟</span>
                    </template>
                </el-input-number>
            </div>
        </el-drawer>

    </div>
</template>

<style scoped>
.home-view {
    display: flex;
    flex-direction: column;
    justify-content: center;
    align-items: center;
    text-align: center;
}

.header {
    font-size: 32px;
    margin-bottom: 10vh;
    margin-top: 10vh;
}

.time-count {
    margin-bottom: 15vh;
}

.setting {
    margin-top: 5vh;
}
</style>