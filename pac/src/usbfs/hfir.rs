#[doc = "Register `HFIR` reader"]
pub type R = crate::R<HfirSpec>;
#[doc = "Register `HFIR` writer"]
pub type W = crate::W<HfirSpec>;
#[doc = "Field `FRIVL` reader - desc FRIVL"]
pub type FrivlR = crate::FieldReader<u16>;
#[doc = "Field `FRIVL` writer - desc FRIVL"]
pub type FrivlW<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
impl R {
    #[doc = "Bits 0:15 - desc FRIVL"]
    #[inline(always)]
    pub fn frivl(&self) -> FrivlR {
        FrivlR::new((self.bits & 0xffff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:15 - desc FRIVL"]
    #[inline(always)]
    pub fn frivl(&mut self) -> FrivlW<'_, HfirSpec> {
        FrivlW::new(self, 0)
    }
}
#[doc = "desc HFIR\n\nYou can [`read`](crate::Reg::read) this register and get [`hfir::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`hfir::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct HfirSpec;
impl crate::RegisterSpec for HfirSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`hfir::R`](R) reader structure"]
impl crate::Readable for HfirSpec {}
#[doc = "`write(|w| ..)` method takes [`hfir::W`](W) writer structure"]
impl crate::Writable for HfirSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets HFIR to value 0xea60"]
impl crate::Resettable for HfirSpec {
    const RESET_VALUE: u32 = 0xea60;
}
