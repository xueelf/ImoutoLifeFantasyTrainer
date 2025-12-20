return {
    {
        action = 'winer',
        code = VK_F1,
        hotkey = 'F1',
        effect = '见面5秒结束战斗',
        description = '战斗阶段直接获得胜利',
    },
    {
        action = 'invincibility',
        code = VK_F2,
        hotkey = 'F2',
        effect = 'AT力场',
        description = '免疫战斗时受到的伤害',
    },
    -- v1 使用的 CE 默认生成模板，v2 用 lua 进行了重构并手绘窗体
    -- 但打包 .exe 后 speedhack 无法加载 DLL 模块，执行 .ct 却一切正常，原因未知
    -- https://github.com/cheat-engine/cheat-engine/issues/1885
    -- CE 为什么这么难用啊，API 文档也不全，开发体验极差 (╯°□°）╯︵ ┻━┻
    -- {
    --     action = 'speed',
    --     code = VK_F3,
    --     hotkey = 'F3',
    --     effect = '三倍速',
    --     description = '游戏变速',
    -- },
    {
        action = 'chronostasis',
        code = VK_F9,
        hotkey = 'F9',
        effect = '砸瓦鲁多',
        description = '停止游戏内的时间流动',
    },
    {
        action = 'ablutomania',
        code = VK_F10,
        hotkey = 'F10',
        effect = '无限洗澡',
        description = '洗澡不受次数限制',
    },
    {
        action = 'teaaholic',
        code = VK_F11,
        hotkey = 'F11',
        effect = '无限泡茶',
        description = '泡茶不受次数限制',
    },
}
