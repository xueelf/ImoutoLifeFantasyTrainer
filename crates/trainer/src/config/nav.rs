//! 静态菜单树。路径由各级 key 拼成，供指针表和 UI 共用。

#[derive(Clone, Copy)]
pub enum NavAction {
    Documentation,
    Repository,
    About,
}

#[derive(Clone, Copy)]
pub enum NavType {
    Group,
    Prompt,
    Check,
    Select,
    Action(NavAction),
}

#[derive(Clone, Copy)]
pub struct NavOption {
    pub caption: &'static str,
    pub value: i32,
    pub read_only: bool,
}

pub struct NavNode {
    pub caption: &'static str,
    pub key: &'static str,
    pub path: String,
    pub read_only: bool,
    pub kind: NavType,
    pub format: Option<&'static str>,
    pub options: Vec<NavOption>,
    pub children: Vec<NavNode>,
}

pub fn tree() -> Vec<NavNode> {
    // 这里只描述菜单结构和可编辑类型，具体数值从游戏内存读取。
    let mut roots = vec![
        NavNode::group(
            "道具",
            "item_list",
            vec![
                NavNode::group(
                    "茶水类",
                    "drinks",
                    vec![
                        prompt("绿茶", "green_tea"),
                        prompt("咖啡", "coffee"),
                        prompt("红茶", "black_tea"),
                        prompt("安眠茶", "sleep_tea"),
                        prompt("壮阳茶", "stimulant"),
                        prompt("媚药", "love_potion"),
                        prompt("避孕药", "after_pill"),
                    ],
                ),
                NavNode::group(
                    "礼物类",
                    "presents",
                    vec![prompt("零食", "sweets"), prompt("礼物", "present")],
                ),
                NavNode::group(
                    "书籍类",
                    "books",
                    vec![
                        prompt("冒险的书", "adventure_book"),
                        prompt("色色的书", "naughty_book"),
                        prompt("小薄本", "doujinshi"),
                    ],
                ),
                NavNode::group(
                    "家具类",
                    "household_items",
                    vec![
                        check("安眠枕", "soft_pillow"),
                        check("羽毛被", "feather_mattress"),
                        check("哑铃", "dumbbell"),
                        check("铁木屐", "iron_sandals"),
                    ],
                ),
                NavNode::group(
                    "H类",
                    "sex_items",
                    vec![
                        prompt_fmt("安全套", "condom", "x{}"),
                        check("跳蛋", "vibrator"),
                        check("按摩棒", "dildo"),
                        check("菊穴按摩棒", "anal_dildo"),
                        check("触手", "tentacle"),
                    ],
                ),
            ],
        ),
        NavNode::group(
            "状态",
            "status",
            vec![
                NavNode::group(
                    "时间",
                    "time",
                    vec![
                        prompt("日", "day"),
                        prompt("时", "hour"),
                        prompt("分", "minute"),
                        select(
                            "周",
                            "week",
                            vec![
                                option("周一", 0, false),
                                option("周二", 1, false),
                                option("周三", 2, false),
                                option("周四", 3, false),
                                option("周五", 4, false),
                                option("周六", 5, false),
                                option("周日", 6, false),
                            ],
                        ),
                    ],
                ),
                select(
                    "方针",
                    "household_policy",
                    vec![
                        option("开始之时", 1, true),
                        option("往常的生活", 2, false),
                        option("打工生活", 3, false),
                        option("便当生活", 4, false),
                        option("节约生活", 5, false),
                        option("读书周", 6, false),
                        option("肉体改造", 7, false),
                        option("大人的阶段", 8, false),
                        option("健康的每一天", 9, false),
                        option("奢侈期", 10, false),
                        option("禁欲生活", 11, false),
                        option("糜烂生活", 12, false),
                        option("烦恼全开", 13, false),
                        option("妹妹生活", 14, false),
                        option("暑假！", 15, true),
                        option("合宿！", 16, true),
                        option("妹妹旅行", 17, false),
                    ],
                ),
                NavNode::group(
                    "妹妹",
                    "imouto",
                    vec![
                        select(
                            "心情",
                            "interest",
                            vec![
                                option("哥哥LOVE", 10, false),
                                option("很开心", 8, false),
                                option("普通", 6, false),
                                option("不开心", 4, false),
                                option("即将爆炸", 2, false),
                                option("爆炸", 0, false),
                            ],
                        ),
                        prompt("信赖", "trust"),
                        prompt("性趣", "mood"),
                        prompt("性欲", "lust"),
                        prompt_readonly("健康", "condition"),
                    ],
                ),
                NavNode::group(
                    "哥哥",
                    "oniichan",
                    vec![
                        prompt("行动力", "energy"),
                        prompt("性欲", "lust"),
                        prompt("精力", "stamina"),
                        prompt("金钱", "money"),
                    ],
                ),
            ],
        ),
        NavNode::group(
            "帮助",
            "help",
            vec![
                action("文档", "documentation", NavAction::Documentation),
                action("源码", "repository", NavAction::Repository),
                action("关于", "about", NavAction::About),
            ],
        ),
    ];

    assign_paths(&mut roots, "");
    roots
}

fn assign_paths(nodes: &mut [NavNode], parent: &str) {
    // 路径会写进指针表和本地化键，确定后不要随意改 key。
    for node in nodes {
        node.path = if parent.is_empty() {
            node.key.to_string()
        } else {
            format!("{parent}.{}", node.key)
        };
        assign_paths(&mut node.children, &node.path);
    }
}

/// 数量字段的显示格式，`{}` 会替换成当前值。
pub fn format_count(format: Option<&str>, value: i32) -> String {
    match format {
        Some(fmt) if !fmt.is_empty() => fmt.replace("{}", &value.to_string()),
        _ => value.to_string(),
    }
}

impl NavNode {
    fn new(caption: &'static str, key: &'static str, kind: NavType) -> Self {
        Self {
            caption,
            key,
            path: String::new(),
            read_only: false,
            kind,
            format: None,
            options: Vec::new(),
            children: Vec::new(),
        }
    }

    fn group(caption: &'static str, key: &'static str, children: Vec<NavNode>) -> Self {
        let mut node = Self::new(caption, key, NavType::Group);
        node.children = children;
        node
    }
}

fn prompt(caption: &'static str, key: &'static str) -> NavNode {
    NavNode::new(caption, key, NavType::Prompt)
}

fn prompt_fmt(caption: &'static str, key: &'static str, format: &'static str) -> NavNode {
    let mut node = prompt(caption, key);
    node.format = Some(format);
    node
}

fn prompt_readonly(caption: &'static str, key: &'static str) -> NavNode {
    let mut node = prompt(caption, key);
    node.read_only = true;
    node
}

fn check(caption: &'static str, key: &'static str) -> NavNode {
    NavNode::new(caption, key, NavType::Check)
}

fn select(caption: &'static str, key: &'static str, options: Vec<NavOption>) -> NavNode {
    let mut node = NavNode::new(caption, key, NavType::Select);
    node.options = options;
    node
}

fn action(caption: &'static str, key: &'static str, command: NavAction) -> NavNode {
    NavNode::new(caption, key, NavType::Action(command))
}

const fn option(caption: &'static str, value: i32, read_only: bool) -> NavOption {
    NavOption {
        caption,
        value,
        read_only,
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::config::pointer;
    use std::collections::HashSet;

    fn walk(nodes: &[NavNode], visit: &mut impl FnMut(&NavNode)) {
        for node in nodes {
            visit(node);
            walk(&node.children, visit);
        }
    }

    #[test]
    fn editable_leaves_have_pointer_config() {
        let nav = tree();
        walk(&nav, &mut |node| {
            if matches!(
                node.kind,
                NavType::Prompt | NavType::Check | NavType::Select
            ) {
                assert!(
                    pointer::lookup(&node.path).is_some(),
                    "missing pointer for {}",
                    node.path
                );
            }
        });
    }

    #[test]
    fn paths_are_unique() {
        let nav = tree();
        let mut seen = HashSet::new();
        walk(&nav, &mut |node| {
            assert!(!node.path.is_empty());
            assert!(
                seen.insert(node.path.clone()),
                "duplicate path {}",
                node.path
            );
        });
    }
}
