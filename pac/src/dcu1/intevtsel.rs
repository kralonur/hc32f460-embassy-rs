#[doc = "Register `INTEVTSEL` reader"]
pub type R = crate::R<IntevtselSpec>;
#[doc = "Register `INTEVTSEL` writer"]
pub type W = crate::W<IntevtselSpec>;
#[doc = "Field `SEL_OP` reader - desc SEL_OP"]
pub type SelOpR = crate::BitReader;
#[doc = "Field `SEL_OP` writer - desc SEL_OP"]
pub type SelOpW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SEL_LS2` reader - desc SEL_LS2"]
pub type SelLs2R = crate::BitReader;
#[doc = "Field `SEL_LS2` writer - desc SEL_LS2"]
pub type SelLs2W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SEL_EQ2` reader - desc SEL_EQ2"]
pub type SelEq2R = crate::BitReader;
#[doc = "Field `SEL_EQ2` writer - desc SEL_EQ2"]
pub type SelEq2W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SEL_GT2` reader - desc SEL_GT2"]
pub type SelGt2R = crate::BitReader;
#[doc = "Field `SEL_GT2` writer - desc SEL_GT2"]
pub type SelGt2W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SEL_LS1` reader - desc SEL_LS1"]
pub type SelLs1R = crate::BitReader;
#[doc = "Field `SEL_LS1` writer - desc SEL_LS1"]
pub type SelLs1W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SEL_EQ1` reader - desc SEL_EQ1"]
pub type SelEq1R = crate::BitReader;
#[doc = "Field `SEL_EQ1` writer - desc SEL_EQ1"]
pub type SelEq1W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SEL_GT1` reader - desc SEL_GT1"]
pub type SelGt1R = crate::BitReader;
#[doc = "Field `SEL_GT1` writer - desc SEL_GT1"]
pub type SelGt1W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SEL_WIN` reader - desc SEL_WIN"]
pub type SelWinR = crate::FieldReader;
#[doc = "Field `SEL_WIN` writer - desc SEL_WIN"]
pub type SelWinW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
impl R {
    #[doc = "Bit 0 - desc SEL_OP"]
    #[inline(always)]
    pub fn sel_op(&self) -> SelOpR {
        SelOpR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - desc SEL_LS2"]
    #[inline(always)]
    pub fn sel_ls2(&self) -> SelLs2R {
        SelLs2R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - desc SEL_EQ2"]
    #[inline(always)]
    pub fn sel_eq2(&self) -> SelEq2R {
        SelEq2R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - desc SEL_GT2"]
    #[inline(always)]
    pub fn sel_gt2(&self) -> SelGt2R {
        SelGt2R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - desc SEL_LS1"]
    #[inline(always)]
    pub fn sel_ls1(&self) -> SelLs1R {
        SelLs1R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - desc SEL_EQ1"]
    #[inline(always)]
    pub fn sel_eq1(&self) -> SelEq1R {
        SelEq1R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - desc SEL_GT1"]
    #[inline(always)]
    pub fn sel_gt1(&self) -> SelGt1R {
        SelGt1R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bits 7:8 - desc SEL_WIN"]
    #[inline(always)]
    pub fn sel_win(&self) -> SelWinR {
        SelWinR::new(((self.bits >> 7) & 3) as u8)
    }
}
impl W {
    #[doc = "Bit 0 - desc SEL_OP"]
    #[inline(always)]
    pub fn sel_op(&mut self) -> SelOpW<'_, IntevtselSpec> {
        SelOpW::new(self, 0)
    }
    #[doc = "Bit 1 - desc SEL_LS2"]
    #[inline(always)]
    pub fn sel_ls2(&mut self) -> SelLs2W<'_, IntevtselSpec> {
        SelLs2W::new(self, 1)
    }
    #[doc = "Bit 2 - desc SEL_EQ2"]
    #[inline(always)]
    pub fn sel_eq2(&mut self) -> SelEq2W<'_, IntevtselSpec> {
        SelEq2W::new(self, 2)
    }
    #[doc = "Bit 3 - desc SEL_GT2"]
    #[inline(always)]
    pub fn sel_gt2(&mut self) -> SelGt2W<'_, IntevtselSpec> {
        SelGt2W::new(self, 3)
    }
    #[doc = "Bit 4 - desc SEL_LS1"]
    #[inline(always)]
    pub fn sel_ls1(&mut self) -> SelLs1W<'_, IntevtselSpec> {
        SelLs1W::new(self, 4)
    }
    #[doc = "Bit 5 - desc SEL_EQ1"]
    #[inline(always)]
    pub fn sel_eq1(&mut self) -> SelEq1W<'_, IntevtselSpec> {
        SelEq1W::new(self, 5)
    }
    #[doc = "Bit 6 - desc SEL_GT1"]
    #[inline(always)]
    pub fn sel_gt1(&mut self) -> SelGt1W<'_, IntevtselSpec> {
        SelGt1W::new(self, 6)
    }
    #[doc = "Bits 7:8 - desc SEL_WIN"]
    #[inline(always)]
    pub fn sel_win(&mut self) -> SelWinW<'_, IntevtselSpec> {
        SelWinW::new(self, 7)
    }
}
#[doc = "desc INTEVTSEL\n\nYou can [`read`](crate::Reg::read) this register and get [`intevtsel::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`intevtsel::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct IntevtselSpec;
impl crate::RegisterSpec for IntevtselSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`intevtsel::R`](R) reader structure"]
impl crate::Readable for IntevtselSpec {}
#[doc = "`write(|w| ..)` method takes [`intevtsel::W`](W) writer structure"]
impl crate::Writable for IntevtselSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets INTEVTSEL to value 0"]
impl crate::Resettable for IntevtselSpec {}
