\ Model schedular
\ LP format - for model browsing. Use MPS format to capture full model detail.
Minimize
 
Subject To
 Wednesday: MA4804_v0 <= 1
 Friday: MA3080_v2 + MA3205_v0 <= 1
 Monday: MA3442_v0 + CIT4100003_v0 <= 1
 Tuesday: MA3080_v0 + MA3080_v1 + MA3080_v2 + MA3442_v0 + MA3442_v1
   + MA4804_v0 + MA4804_v1 <= 1
 Thursday: MA3205_v0 + MA4804_v1 + CIT4100003_v0 <= 1
 Wednesday_14_30_00: MA4804_v0 <= 1
 Friday_08_45_00: MA3080_v2 <= 1
 Tuesday_15_00_00: MA3442_v0 + MA3442_v1 <= 1
 Monday_11_30_00: MA3442_v0 <= 1
 Tuesday_11_30_00: MA4804_v0 + MA4804_v1 <= 1
 Monday_13_15_00: CIT4100003_v0 <= 1
 Tuesday_09_45_00: MA3080_v0 <= 1
 Monday_16_15_00: CIT4100003_v0 <= 1
 Monday_13_00_00: CIT4100003_v0 <= 1
 Tuesday_16_30_00: MA3080_v0 + MA3080_v1 + MA3080_v2 + MA3442_v1 <= 1
 Tuesday_13_15_00: MA3080_v1 <= 1
 Monday_16_45_00: CIT4100003_v0 <= 1
 Tuesday_17_15_00: MA3080_v0 + MA3080_v1 + MA3080_v2 + MA3442_v1 <= 1
 Friday_09_15_00: MA3080_v2 <= 1
 Wednesday_14_45_00: MA4804_v0 <= 1
 Friday_11_30_00: MA3205_v0 <= 1
 Monday_11_00_00: MA3442_v0 <= 1
 Thursday_10_30_00: MA4804_v1 + CIT4100003_v0 <= 1
 Thursday_09_00_00: MA3205_v0 <= 1
 Monday_10_45_00: MA3442_v0 <= 1
 Friday_10_45_00: MA3205_v0 <= 1
 Friday_10_30_00: MA3205_v0 <= 1
 Tuesday_10_30_00: MA4804_v0 + MA4804_v1 <= 1
 Friday_08_30_00: MA3080_v2 <= 1
 Thursday_09_45_00: MA3205_v0 <= 1
 Tuesday_09_15_00: MA3080_v0 <= 1
 Tuesday_10_15_00: MA4804_v0 + MA4804_v1 <= 1
 Tuesday_11_00_00: MA4804_v0 + MA4804_v1 <= 1
 Thursday_08_45_00: MA3205_v0 <= 1
 Thursday_08_30_00: MA3205_v0 <= 1
 Tuesday_13_30_00: MA3080_v1 <= 1
 Friday_09_00_00: MA3080_v2 <= 1
 Tuesday_09_30_00: MA3080_v0 <= 1
 Friday_11_00_00: MA3205_v0 <= 1
 Tuesday_11_15_00: MA4804_v0 + MA4804_v1 <= 1
 Monday_12_15_00: CIT4100003_v0 <= 1
 Thursday_11_15_00: MA4804_v1 + CIT4100003_v0 <= 1
 Monday_12_30_00: CIT4100003_v0 <= 1
 Tuesday_16_45_00: MA3080_v0 + MA3080_v1 + MA3080_v2 + MA3442_v1 <= 1
 Thursday_09_15_00: MA3205_v0 <= 1
 Tuesday_17_00_00: MA3080_v0 + MA3080_v1 + MA3080_v2 + MA3442_v1 <= 1
 Tuesday_10_45_00: MA4804_v0 + MA4804_v1 <= 1
 Tuesday_16_15_00: MA3080_v0 + MA3080_v1 + MA3080_v2 + MA3442_v1 <= 1
 Friday_09_30_00: MA3080_v2 <= 1
 Wednesday_15_15_00: MA4804_v0 <= 1
 Monday_10_15_00: MA3442_v0 <= 1
 Thursday_11_00_00: MA4804_v1 + CIT4100003_v0 <= 1
 Tuesday_12_30_00: MA3080_v1 <= 1
 Thursday_11_30_00: MA4804_v1 + CIT4100003_v0 <= 1
 Thursday_10_45_00: MA4804_v1 + CIT4100003_v0 <= 1
 Tuesday_12_45_00: MA3080_v1 <= 1
 Thursday_10_15_00: MA4804_v1 + CIT4100003_v0 <= 1
 Tuesday_14_30_00: MA3442_v0 + MA3442_v1 <= 1
 Monday_12_45_00: CIT4100003_v0 <= 1
 Tuesday_13_00_00: MA3080_v1 <= 1
 Friday_10_15_00: MA3205_v0 <= 1
 Monday_17_15_00: CIT4100003_v0 <= 1
 Monday_10_30_00: MA3442_v0 <= 1
 Friday_11_15_00: MA3205_v0 <= 1
 Tuesday_16_00_00: MA3080_v0 + MA3080_v1 + MA3080_v2 <= 1
 Wednesday_15_00_00: MA4804_v0 <= 1
 Thursday_09_30_00: MA3205_v0 <= 1
 Monday_17_30_00: CIT4100003_v0 <= 1
 Tuesday_15_30_00: MA3442_v0 + MA3442_v1 <= 1
 Wednesday_14_15_00: MA4804_v0 <= 1
 Tuesday_09_00_00: MA3080_v0 <= 1
 Wednesday_15_30_00: MA4804_v0 <= 1
 Tuesday_14_45_00: MA3442_v0 + MA3442_v1 <= 1
 Tuesday_15_45_00: MA3442_v0 + MA3442_v1 <= 1
 Monday_13_30_00: CIT4100003_v0 <= 1
 Tuesday_17_45_00: MA3080_v0 + MA3080_v1 + MA3080_v2 <= 1
 Tuesday_17_30_00: MA3080_v0 + MA3080_v1 + MA3080_v2 + MA3442_v1 <= 1
 Monday_17_00_00: CIT4100003_v0 <= 1
 Tuesday_08_30_00: MA3080_v0 <= 1
 Tuesday_08_45_00: MA3080_v0 <= 1
 Tuesday_15_15_00: MA3442_v0 + MA3442_v1 <= 1
 Friday_09_45_00: MA3080_v2 <= 1
 Tuesday_14_15_00: MA3442_v0 + MA3442_v1 <= 1
 Tuesday_14_00_00: MA3442_v0 + MA3442_v1 <= 1
 Monday_16_30_00: CIT4100003_v0 <= 1
 Tuesday_12_15_00: MA3080_v1 <= 1
 Monday_11_15_00: MA3442_v0 <= 1
 min_ects: 5 MA3080_v0 + 5 MA3080_v1 + 5 MA3080_v2 + 6 MA3205_v0
   + 5 MA3442_v0 + 5 MA3442_v1 + 6 MA4804_v0 + 6 MA4804_v1
   + 9 CIT4100003_v0 >= 13
Bounds
Binaries
 MA3080_v0 MA3080_v1 MA3080_v2 MA3205_v0 MA3442_v0 MA3442_v1 MA4804_v0
 MA4804_v1 CIT4100003_v0
End
