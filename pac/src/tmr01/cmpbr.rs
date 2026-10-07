#[doc = "Register `CMPBR` reader"]
pub type R = crate::R<CmpbrSpec>;
#[doc = "Register `CMPBR` writer"]
pub type W = crate::W<CmpbrSpec>;
#[doc = "Field `CMPB` reader - desc CMPB"]
pub type CmpbR = crate::FieldReader<u16>;
#[doc = "Field `CMPB` writer - desc CMPB"]
pub type CmpbW<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
impl R {
    #[doc = "Bits 0:15 - desc CMPB"]
    #[inline(always)]
    pub fn cmpb(&self) -> CmpbR {
        CmpbR::new((self.bits & 0xffff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:15 - desc CMPB"]
    #[inline(always)]
    pub fn cmpb(&mut self) -> CmpbW<'_, CmpbrSpec> {
        CmpbW::new(self, 0)
    }
}
#[doc = "desc CMPBR\n\nYou can [`read`](crate::Reg::read) this register and get [`cmpbr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cmpbr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CmpbrSpec;
impl crate::RegisterSpec for CmpbrSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`cmpbr::R`](R) reader structure"]
impl crate::Readable for CmpbrSpec {}
#[doc = "`write(|w| ..)` method takes [`cmpbr::W`](W) writer structure"]
impl crate::Writable for CmpbrSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets CMPBR to value 0xffff"]
impl crate::Resettable for CmpbrSpec {
    const RESET_VALUE: u32 = 0xffff;
}
