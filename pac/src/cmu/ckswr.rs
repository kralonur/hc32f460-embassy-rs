#[doc = "Register `CKSWR` reader"]
pub type R = crate::R<CkswrSpec>;
#[doc = "Register `CKSWR` writer"]
pub type W = crate::W<CkswrSpec>;
#[doc = "Field `CKSW` reader - desc CKSW"]
pub type CkswR = crate::FieldReader;
#[doc = "Field `CKSW` writer - desc CKSW"]
pub type CkswW<'a, REG> = crate::FieldWriter<'a, REG, 3>;
impl R {
    #[doc = "Bits 0:2 - desc CKSW"]
    #[inline(always)]
    pub fn cksw(&self) -> CkswR {
        CkswR::new(self.bits & 7)
    }
}
impl W {
    #[doc = "Bits 0:2 - desc CKSW"]
    #[inline(always)]
    pub fn cksw(&mut self) -> CkswW<'_, CkswrSpec> {
        CkswW::new(self, 0)
    }
}
#[doc = "desc CKSWR\n\nYou can [`read`](crate::Reg::read) this register and get [`ckswr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ckswr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CkswrSpec;
impl crate::RegisterSpec for CkswrSpec {
    type Ux = u8;
}
#[doc = "`read()` method returns [`ckswr::R`](R) reader structure"]
impl crate::Readable for CkswrSpec {}
#[doc = "`write(|w| ..)` method takes [`ckswr::W`](W) writer structure"]
impl crate::Writable for CkswrSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets CKSWR to value 0x01"]
impl crate::Resettable for CkswrSpec {
    const RESET_VALUE: u8 = 0x01;
}
