-- Your SQL goes here
CREATE TABLE `problem` (
  `id` bigint NOT NULL AUTO_INCREMENT COMMENT '问题ID',
  `stem` varchar(1024) COLLATE utf8mb4_general_ci NOT NULL COMMENT '题干',
  `problem_type` varchar(30) COLLATE utf8mb4_general_ci NOT NULL COMMENT '问题类型 S:单选题， M:多选题  TF: 判断题',
  `option_a` varchar(1024) COLLATE utf8mb4_general_ci DEFAULT NULL COMMENT '选项A',
  `option_b` varchar(1024) COLLATE utf8mb4_general_ci DEFAULT NULL COMMENT '选项B',
  `option_c` varchar(1024) COLLATE utf8mb4_general_ci DEFAULT NULL COMMENT '选项C',
  `option_d` varchar(1024) COLLATE utf8mb4_general_ci DEFAULT NULL COMMENT '选项D',
  `option_e` varchar(1024) COLLATE utf8mb4_general_ci DEFAULT NULL COMMENT '选项E',
  `option_f` varchar(1024) COLLATE utf8mb4_general_ci DEFAULT NULL COMMENT '选项F',
  `answer` varchar(32) COLLATE utf8mb4_general_ci not NULL COMMENT '答案',
  `status` int DEFAULT '0' not null COMMENT '帐号状态（0正常 1停用）',
  `is_delete` char(1) COLLATE utf8mb4_general_ci DEFAULT 'N' not null COMMENT '删除标志（N代表正常 Y代表删除）',
  `create_by` varchar(64) CHARACTER SET utf8mb4 COLLATE utf8mb4_general_ci DEFAULT '' not null COMMENT '创建者',
  `create_time` datetime NOT NULL COMMENT '创建时间',
  `update_by` varchar(64) CHARACTER SET utf8mb4 COLLATE utf8mb4_general_ci DEFAULT '' not null COMMENT '更新者',
  `update_time` datetime NOT NULL COMMENT '更新时间',
  PRIMARY KEY (`id`) USING BTREE
) ENGINE=InnoDB DEFAULT CHARSET=utf8mb4 COLLATE=utf8mb4_general_ci ROW_FORMAT=DYNAMIC COMMENT='问题表'