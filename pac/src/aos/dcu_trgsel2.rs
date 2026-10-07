#[doc = "Register `DCU_TRGSEL2` reader"]
pub type R = crate::R<DcuTrgsel2Spec>;
#[doc = "Register `DCU_TRGSEL2` writer"]
pub type W = crate::W<DcuTrgsel2Spec>;
#[doc = "Field `TRGSEL` reader - desc TRGSEL"]
pub type TrgselR = crate::FieldReader<u16>;
#[doc = "Field `TRGSEL` writer - desc TRGSEL"]
pub type TrgselW<'a, REG> = crate::FieldWriter<'a, REG, 9, u16>;
#[doc = "Field `COMEN` reader - desc COMEN"]
pub type ComenR = crate::FieldReader;
#[doc = "Field `COMEN` writer - desc COMEN"]
pub type ComenW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
impl R {
    #[doc = "Bits 0:8 - desc TRGSEL"]
    #[inline(always)]
    pub fn trgsel(&self) -> TrgselR {
        TrgselR::new((self.bits & 0x01ff) as u16)
    }
    #[doc = "Bits 30:31 - desc COMEN"]
    #[inline(always)]
    pub fn comen(&self) -> ComenR {
        ComenR::new(((self.bits >> 30) & 3) as u8)
    }
}
impl W {
    #[doc = "Bits 0:8 - desc TRGSEL"]
    #[inline(always)]
    pub fn trgsel(&mut self) -> TrgselW<'_, DcuTrgsel2Spec> {
        TrgselW::new(self, 0)
    }
    #[doc = "Bits 30:31 - desc COMEN"]
    #[inline(always)]
    pub fn comen(&mut self) -> ComenW<'_, DcuTrgsel2Spec> {
        ComenW::new(self, 30)
    }
}
#[doc = "desc DCU_TRGSEL2\n\nYou can [`read`](crate::Reg::read) this register and get [`dcu_trgsel2::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dcu_trgsel2::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct DcuTrgsel2Spec;
impl crate::RegisterSpec for DcuTrgsel2Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`dcu_trgsel2::R`](R) reader structure"]
impl crate::Readable for DcuTrgsel2Spec {}
#[doc = "`write(|w| ..)` method takes [`dcu_trgsel2::W`](W) writer structure"]
impl crate::Writable for DcuTrgsel2Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets DCU_TRGSEL2 to value 0x01ff"]
impl crate::Resettable for DcuTrgsel2Spec {
    const RESET_VALUE: u32 = 0x01ff;
}
