#[doc = "Register `FLAG` reader"]
pub type R = crate::R<FlagSpec>;
#[doc = "Field `FLAG_OP` reader - desc FLAG_OP"]
pub type FlagOpR = crate::BitReader;
#[doc = "Field `FLAG_LS2` reader - desc FLAG_LS2"]
pub type FlagLs2R = crate::BitReader;
#[doc = "Field `FLAG_EQ2` reader - desc FLAG_EQ2"]
pub type FlagEq2R = crate::BitReader;
#[doc = "Field `FLAG_GT2` reader - desc FLAG_GT2"]
pub type FlagGt2R = crate::BitReader;
#[doc = "Field `FLAG_LS1` reader - desc FLAG_LS1"]
pub type FlagLs1R = crate::BitReader;
#[doc = "Field `FLAG_EQ1` reader - desc FLAG_EQ1"]
pub type FlagEq1R = crate::BitReader;
#[doc = "Field `FLAG_GT1` reader - desc FLAG_GT1"]
pub type FlagGt1R = crate::BitReader;
impl R {
    #[doc = "Bit 0 - desc FLAG_OP"]
    #[inline(always)]
    pub fn flag_op(&self) -> FlagOpR {
        FlagOpR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - desc FLAG_LS2"]
    #[inline(always)]
    pub fn flag_ls2(&self) -> FlagLs2R {
        FlagLs2R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - desc FLAG_EQ2"]
    #[inline(always)]
    pub fn flag_eq2(&self) -> FlagEq2R {
        FlagEq2R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - desc FLAG_GT2"]
    #[inline(always)]
    pub fn flag_gt2(&self) -> FlagGt2R {
        FlagGt2R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - desc FLAG_LS1"]
    #[inline(always)]
    pub fn flag_ls1(&self) -> FlagLs1R {
        FlagLs1R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - desc FLAG_EQ1"]
    #[inline(always)]
    pub fn flag_eq1(&self) -> FlagEq1R {
        FlagEq1R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - desc FLAG_GT1"]
    #[inline(always)]
    pub fn flag_gt1(&self) -> FlagGt1R {
        FlagGt1R::new(((self.bits >> 6) & 1) != 0)
    }
}
#[doc = "desc FLAG\n\nYou can [`read`](crate::Reg::read) this register and get [`flag::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct FlagSpec;
impl crate::RegisterSpec for FlagSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`flag::R`](R) reader structure"]
impl crate::Readable for FlagSpec {}
#[doc = "`reset()` method sets FLAG to value 0"]
impl crate::Resettable for FlagSpec {}
